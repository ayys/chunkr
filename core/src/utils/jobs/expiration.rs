use crate::utils::clients;
use crate::utils::storage::services::delete_folder;
use futures::future::try_join_all;

pub async fn expire() -> Result<(), Box<dyn std::error::Error>> {
    let client = clients::get_pg_client().await?;
    let expired_tasks = client
        .query(
            "SELECT task_id, image_folder_location 
            FROM tasks 
            WHERE expires_at < CURRENT_TIMESTAMP 
            AND finished_at < CURRENT_TIMESTAMP 
            AND status in ('Succeeded', 'Failed', 'Cancelled')",
            &[],
        )
        .await?;

    let deletion_futures = expired_tasks.iter().map(|row| {
        let image_folder: String = row
            .get::<_, Option<String>>("image_folder_location")
            .unwrap_or_default();
        let folder_location = image_folder
            .rsplit_once('/')
            .map(|(base, _)| base.to_string())
            .unwrap_or(image_folder.clone());
        async move {
            if let Err(e) = delete_folder(&folder_location).await {
                println!("Error deleting S3 folder {folder_location}: {e:?}");
            }
            Ok::<_, Box<dyn std::error::Error>>(())
        }
    });

    try_join_all(deletion_futures).await?;

    let rows_affected = client
        .execute(
            "DELETE FROM tasks 
            WHERE expires_at < CURRENT_TIMESTAMP 
            AND finished_at < CURRENT_TIMESTAMP 
            AND status in ('Succeeded', 'Failed', 'Cancelled')",
            &[],
        )
        .await?;

    println!("Deleted {rows_affected} expired tasks");
    Ok(())
}
