use crate::domain::job::job_escrow_id::JobEscrowId;
use crate::domain::job::manifest_url::ManifestUrl;

pub struct NewJob {
    pub job_escrow_id: JobEscrowId,
    pub manifest_url: ManifestUrl,
}
