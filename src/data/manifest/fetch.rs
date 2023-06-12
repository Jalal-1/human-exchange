use crate::domain::manifest::fetched_manifest::FetchedManifest;
use reqwest::Error;

pub async fn fetch_manifest(url: &str) -> Result<FetchedManifest, Error> {
    let manifest_url = url.to_string(); // Save the manifest URL before querying
    let response = reqwest::get(url).await?;
    let mut fetched_manifest: FetchedManifest = response.json().await?;
    fetched_manifest.manifestUrl = Some(manifest_url); // Assign the saved manifest URL
    Ok(fetched_manifest)
}
