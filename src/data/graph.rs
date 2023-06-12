use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GraphJob {
    pub id: String,
    pub timestamp: String,
    pub manifestUrl: Option<String>,
}

/// Graph response object for [`launchedEscrows`] query.
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Data {
    launchedEscrows: Vec<GraphJob>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Error {
    message: String,
}

// Graph query response
#[derive(Debug, Deserialize)]
struct QueryResponse {
    data: Option<Data>,
    // errors: Option<Vec<Error>>,
}

pub async fn get_escrows_from_graph(
    last_posted: Option<i64>,
) -> Result<Vec<GraphJob>, Box<dyn std::error::Error>> {
    let filter = match last_posted {
        Some(time) => format!(", where: {{ posted_gt: {} }}", time),
        None => String::new(),
    };

    let query = format!(
        r#"
        {{
            launchedEscrows(first: 500, orderBy: timestamp, orderDirection: desc{} ) {{
                id
                timestamp
                manifestUrl
           }}
        }}
    "#,
        filter,
    );

    let client = reqwest::Client::new();

    let res = client
        .post("https://api.thegraph.com/subgraphs/name/humanprotocol/mumbai-v1")
        .json(&json!({
            "query": query,
        }))
        .send()
        .await?
        .json::<QueryResponse>()
        .await?;

    match res.data {
        None => Err("Response from server did not contain any data".into()),
        Some(data) => Ok(data.launchedEscrows),
    }
}

pub async fn save_escrows_to_db(
    pool: &PgPool,
    escrows: Vec<GraphJob>,
) -> Result<(), Box<dyn std::error::Error>> {
    for escrow in escrows {
        let timestamp = escrow.timestamp.parse::<i64>().unwrap_or(0);
        sqlx::query!(
            r#"
            INSERT INTO jobs(
            job_id,
            job_escrow_id,
            posted,
            manifest_url
            )
            VALUES ($1, $2, $3, $4)
            "#,
            Uuid::new_v4(),
            escrow.id,
            timestamp,
            escrow.manifestUrl
        )
            .execute(pool)
            .await?;
    }
    Ok(())
}


pub async fn get_last_fetched_escrow_id_time(
    pool: &PgPool,
) -> Result<Option<i64>, Box<dyn std::error::Error>> {
    let result = sqlx::query!("SELECT posted FROM jobs ORDER BY posted DESC LIMIT 1")
        .fetch_optional(pool)
        .await?;
        
    match result {
        Some(last_escrow) => Ok(last_escrow.posted),
        None => Ok(None),
    }
}



pub async fn download_graph_jobs(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let last_posted = get_last_fetched_escrow_id_time(pool).await.unwrap_or(None);
    println!("Last posted timestamp: {:?}", last_posted); // Add this line for logging

    let escrows = get_escrows_from_graph(last_posted).await?;
    save_escrows_to_db(pool, escrows).await?;
    Ok(())
}
