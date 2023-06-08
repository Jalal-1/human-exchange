use crate::domain::manifest::description::Description;
use crate::domain::manifest::fortunes_required::FortunesRequired;
use crate::domain::manifest::manifest_escrow_id::ManifestEscrowId;
use crate::domain::manifest::title::Title;

pub struct NewManifest {
    pub manifest_escrow_id: ManifestEscrowId,
    pub title: Title,
    pub description: Description,
    pub fortunes_required: FortunesRequired,
}
