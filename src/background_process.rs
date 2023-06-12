use sqlx::PgPool;
use tokio::time::{sleep, Duration};

use crate::data::{download_graph_jobs, fetch::fetch_manifest, save::save_manifest};
use crate::domain::job::Job;

pub async fn background_worker(pool: PgPool) {
    loop {
        // Fetch and save jobs every 60 seconds
        match download_graph_jobs(&pool).await {
            Ok(_) => println!("Successfully downloaded jobs."),
            Err(e) => eprintln!("Failed to download jobs: {}", e),
        }

        // Fetch jobs that are missing manifests.
        let jobs_missing_manifests = match fetch_jobs_missing_manifests(&pool).await {
            Ok(jobs) => jobs,
            Err(e) => {
                eprintln!("Failed to fetch jobs missing manifests: {}", e);
                continue;
            },
        };

        // Fetch and save the manifest for each job.
        for job in jobs_missing_manifests {
            if let Some(manifest_url) = &job.manifest_url {
                match fetch_manifest(manifest_url).await {
                    Ok(fetched_manifest) => {
                        if let Err(e) = save_manifest(&pool, fetched_manifest, &job.job_escrow_id).await {
                            eprintln!("Failed to save manifest for job {}: {}", job.job_escrow_id, e);
                        }
                    }
                    Err(e) => eprintln!("Failed to fetch manifest for job {}: {}", job.job_escrow_id, e),
                }
            }
        }

        sleep(Duration::from_secs(6)).await;
    }
}


pub async fn fetch_jobs_missing_manifests(pool: &PgPool) -> Result<Vec<Job>, sqlx::Error> {
    let jobs = sqlx::query_as!(
        Job,
        r#"
        SELECT jobs.job_escrow_id, jobs.manifest_url, jobs.posted
        FROM jobs
        LEFT JOIN manifests ON jobs.job_escrow_id = manifests.manifest_escrow_id
        WHERE manifests.manifest_escrow_id IS NULL AND jobs.manifest_url IS NOT NULL
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(jobs)
}

