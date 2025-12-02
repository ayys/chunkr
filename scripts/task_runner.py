import json
from pprint import pprint
import httpx
import typer
from pathlib import Path
from typing import Optional

chunkr_base_url = "http://localhost:8000"

app = typer.Typer()


@app.command()
def health():
    """Check health"""
    try:
        response = httpx.get(f"{chunkr_base_url}/health")
    except Exception:
        print("Server unhealthy!")
        return
    if response.status_code == 200:
        print(response.text)
        return
    print(f"Server unhealthy\n{response.text}")


@app.command()
def run(filename: Path):
    """Run task"""
    path = Path(filename).resolve()
    if not path.exists():
        raise Exception(f"File does not exist: {filename}")
    if not path.is_file():
        raise Exception(f"Path is not a file: {filename}")

    print(f"Run task on filename {filename}")


@app.command()
def create_task(filename: Path):
    """Create a new task by uploading a file to the chunkr server"""
    path = Path(filename).resolve()
    if not path.exists():
        raise typer.BadParameter(f"File does not exist: {filename}")
    if not path.is_file():
        raise typer.BadParameter(f"Path is not a file: {filename}")

    print(f"Creating task for file: {path}")

    try:
        with open(path, "rb") as f:
            files = {"file": (path.name, f)}
            response = httpx.post(
                f"{chunkr_base_url}/api/v1/task",
                files=files,
                timeout=60.0,
            )

        if response.status_code == 200:
            task_response = response.json()
            task_id = task_response.get("task_id")
            status = task_response.get("status")
            print(f"Task created successfully!")
            print(f"  Task ID: {task_id}")
            print(f"  Status: {status}")
            return task_response
        else:
            print(f"Failed to create task: {response.status_code}")
            print(response.text)
            raise typer.Exit(code=1)

    except httpx.RequestError as e:
        print(f"Error connecting to server: {e}")
        raise typer.Exit(code=1)


@app.command()
def get_task(
    task_id: str,
    include_chunks: bool = typer.Option(False, help="Include chunks in output"),
):
    """Get a task by its ID"""
    params = {}
    if include_chunks:
        params["include_chunks"] = "true"

    try:
        response = httpx.get(
            f"{chunkr_base_url}/api/v1/task/{task_id}",
            params=params,
            timeout=30.0,
        )

        if response.status_code == 200:
            task_response = response.json()
            pprint(task_response)
            return task_response
        elif response.status_code == 404:
            print(f"Task not found: {task_id}")
            raise typer.Exit(code=1)
        else:
            print(f"Failed to get task: {response.status_code}")
            print(response.text)
            raise typer.Exit(code=1)

    except httpx.RequestError as e:
        print(f"Error connecting to server: {e}")
        raise typer.Exit(code=1)


@app.command()
def get_tasks(
    page: Optional[int] = typer.Option(None, help="Page number"),
    limit: Optional[int] = typer.Option(None, help="Number of tasks per page"),
    include_chunks: bool = typer.Option(False, help="Include chunks in output"),
):
    """Get a list of tasks"""
    params = {}
    if page is not None:
        params["page"] = page
    if limit is not None:
        params["limit"] = limit
    if include_chunks:
        params["include_chunks"] = "true"

    try:
        response = httpx.get(
            f"{chunkr_base_url}/api/v1/tasks",
            params=params,
            timeout=30.0,
        )

        if response.status_code == 200:
            tasks = response.json()
            if page is not None:
                print(f"Page {page} (limit: {limit}) - {len(tasks)} task(s)")
            else:
                print(f"Found {len(tasks)} task(s)")
            for task in tasks:
                task_id = task.get("task_id")
                status = task.get("status")
                file_name = task.get("file_name")
                print(f"  [{status}] {task_id} - {file_name}")
            return tasks
        elif response.status_code == 400:
            print(f"Bad request: {response.text}")
            raise typer.Exit(code=1)
        else:
            print(f"Failed to get tasks: {response.status_code}")
            print(response.text)
            raise typer.Exit(code=1)

    except httpx.RequestError as e:
        print(f"Error connecting to server: {e}")
        raise typer.Exit(code=1)


def main():
    app()
