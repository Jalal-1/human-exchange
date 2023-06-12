use crate::domain::manifest::fetched_manifest::FetchedManifest;
use reqwest::Error;

pub async fn fetch_manifest(url: &str) -> Result<FetchedManifest, Error> {
    let response = reqwest::get(url).await?;
    let fetched_manifest: FetchedManifest = response.json().await?;
    Ok(fetched_manifest)
}
