
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FetchedManifest {
    pub manifest_escrow_id: Option<String>,
    pub title: String,
    pub description: String,
    pub fortunes_required: Option<i32>,
}
