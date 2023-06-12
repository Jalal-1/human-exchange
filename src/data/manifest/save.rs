use sqlx::{Error, PgPool};
use uuid::Uuid;
use crate::domain::manifest::fetched_manifest::FetchedManifest;

pub async fn save_manifest(pool: &PgPool, manifest: FetchedManifest) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO manifests (
            manifest_id,
            manifest_escrow_id,
            title,
            description,
            fortunes_required)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        Uuid::new_v4(),
        manifest.manifest_escrow_id,
        manifest.title,
        manifest.description,
        manifest.fortunes_required
    )
    .execute(pool)
    .await?;

    Ok(())
}
