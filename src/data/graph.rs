use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GraphJob {
    pub id: String,
}

/// Graph response object for [`launchedEscrows`] query.
#[derive(Debug, Deserialize)]
struct Data {
    launchedEscrows: Vec<GraphJob>,
}

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

pub async fn get_escrows_from_graph() -> Result<Vec<GraphJob>, Box<dyn std::error::Error>> {
    let query = r#"
        {
            launchedEscrows(first: 500, orderBy: timestamp, orderDirection: desc) {
                id
           }
        }
    "#;

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
