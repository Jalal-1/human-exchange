pub mod fetch;
pub mod save;
pub mod submit;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Manifest {
    pub manifest_id: String,
    pub manifest_url: Option<String>,
    pub manifest_escrow_id: String,
    pub title: String,
    pub description: String,
    pub fortunes_required: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct WorkerManifest {
    pub title: String,
    pub description: String,
    pub manifest_escrow_id: String
}