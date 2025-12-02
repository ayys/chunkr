use crate::models::task::Task;

pub async fn delete_task(task_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let task = Task::get(&task_id).await.map_err(|_| "Task not found")?;
    task.delete().await?;
    Ok(())
}
