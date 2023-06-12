use sqlx::{Error, PgPool};
use uuid::Uuid;
use crate::domain::manifest::fetched_manifest::FetchedManifest;

pub async fn save_manifest(pool: &PgPool, manifest: FetchedManifest, escrow_id: &str) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO manifests (
            manifest_id,
            manifest_escrow_id,
            manifest_url,
            title,
            description,
            fortunes_required
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        escrow_id,
        manifest.manifestUrl,
        manifest.title,
        manifest.description,
        manifest.fortunes_required
    )
    .execute(pool)
    .await?;

    Ok(())
}
