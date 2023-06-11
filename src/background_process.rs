use sqlx::PgPool;
use tokio::time::{sleep, Duration};
use crate::data::download_graph_jobs;

pub async fn background_worker(pool: PgPool) {
    loop {
        // Fetch and save jobs every 60 seconds
        match download_graph_jobs(&pool).await {
            Ok(_) => println!("Successfully downloaded jobs."),
            Err(e) => eprintln!("Failed to download jobs: {}", e),
        }
        sleep(Duration::from_secs(60)).await;
    }
}