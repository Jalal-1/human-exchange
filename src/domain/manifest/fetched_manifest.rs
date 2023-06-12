#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchedManifest {
    pub manifest_escrow_id: Option<String>, // Update the field name to match the JSON field name
    pub manifestUrl: Option<String>,
    pub title: String,
    pub description: String,
    pub fortunes_required: Option<i32>,
}
