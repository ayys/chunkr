use crate::models::task::{Task, TaskQuery, TaskResponse};
use chrono::{DateTime, Utc};

pub async fn get_task(
    task_id: String,
    task_query: TaskQuery,
) -> Result<TaskResponse, Box<dyn std::error::Error>> {
    let task = Task::get(&task_id).await.map_err(|_| "Task not found")?;
    let expires_at: Option<DateTime<Utc>> = task.expires_at;
    if expires_at.is_some() && expires_at.unwrap() < Utc::now() {
        return Err("Task expired".into());
    }
    task.to_task_response(task_query.include_chunks, task_query.base64_urls)
        .await
}
