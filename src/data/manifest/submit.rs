use sqlx::{PgPool, Row};
use reqwest::Client;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use super::{Manifest, WorkerManifest};

pub async fn submit_manifest_worker(pool: PgPool) {
    let client = Client::new();

    loop {
        let manifests = match get_manifests(&pool).await {
            Ok(manifests) => manifests,
            Err(e) => {
                eprintln!("Failed to fetch manifests: {}", e);
                continue;
            }
        };

        for manifest in manifests {
            let worker_manifest = convert_to_worker_manifest(&manifest);

            let worker_endpoint = "https://hmt.ai/worker";
            println!("Sending manifest to worker: {:#?}", worker_manifest);

            match send_manifest_to_worker(&client, &worker_endpoint, &worker_manifest).await {
                Ok(response) => {
                    if let Err(e) = save_worker_response(&pool, &manifest, &response, &worker_endpoint).await {
                        eprintln!("Failed to save worker response: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to send manifest to worker: {}", e);
                }
            }
        }

        sleep(Duration::from_secs(60)).await;
    }
}

// Get manifests
async fn get_manifests(pool: &PgPool) -> Result<Vec<Manifest>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT 
            manifest_id::text, 
            manifest_url, 
            manifest_escrow_id::text, 
            title::text, 
            description::text, 
            fortunes_required
        FROM manifests
        "#
    )
    .fetch_all(pool)
    .await?;

    let manifests = rows.into_iter().map(|row| {
        let manifest_id: String = row.get("manifest_id");
        let manifest_url: Option<String> = row.get("manifest_url");
        let manifest_escrow_id: String = row.get("manifest_escrow_id");
        let title: String = row.get("title");
        let description: String = row.get("description");
        let fortunes_required: Option<i32> = row.get("fortunes_required");

        Manifest {
            manifest_id,
            manifest_url,
            manifest_escrow_id,
            title,
            description,
            fortunes_required,
        }
    }).collect();

    Ok(manifests)
}




fn convert_to_worker_manifest(manifest: &Manifest) -> WorkerManifest {
    WorkerManifest {
        title: manifest.title.clone(),
        description: manifest.description.clone(),
        manifest_escrow_id: manifest.manifest_escrow_id.clone(),
    }
}

async fn send_manifest_to_worker(
    client: &Client,
    worker_endpoint: &str,
    manifest: &WorkerManifest,
) -> Result<String, reqwest::Error> {
    // Implement your logic to send the worker manifest to the worker endpoint
    // Use the `client` to make the HTTP request to the worker endpoint
    // Serialize the `manifest` object to JSON and send it in the request body
    // Return the response body as a String
    // Example:
    let response = client
        .post(worker_endpoint)
        .json(manifest)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

async fn save_worker_response(
    pool: &PgPool,
    manifest: &Manifest,
    response: &str,
    worker_endpoint: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO worker_responses (response_id, escrow_id, response, worker) VALUES ($1, $2, $3, $4)",
        Uuid::new_v4(),
        manifest.manifest_escrow_id,
        response,
        worker_endpoint,
    )
    .execute(pool)
    .await?;

    Ok(())
}
