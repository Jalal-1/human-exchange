mod get_job;
mod job_escrow_id;
mod manifest_url;
mod new_job;

pub use get_job::GetJob;
pub use job_escrow_id::JobEscrowId;
pub use manifest_url::ManifestUrl;
pub use new_job::NewJob;

pub struct Job {
    pub job_escrow_id: String,
    pub manifest_url: Option<String>,
    pub posted: Option<i64>,
}
