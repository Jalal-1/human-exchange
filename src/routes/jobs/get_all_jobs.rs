use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Job {
    pub job_id: Uuid,
    pub job_escrow_id: String,
    pub manifest_url: Option<String>,
    pub posted: Option<i64>,
}

pub async fn get_all_jobs(pool: web::Data<PgPool>) -> HttpResponse {
    let jobs = get_jobs(&pool).await;
    match jobs {
        Ok(jobs) => HttpResponse::Ok().json(jobs),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_jobs(pool: &PgPool) -> Result<Vec<Job>, sqlx::Error> {
    let jobs = sqlx::query_as!(
        Job,
        r#"
        SELECT job_id, job_escrow_id, manifest_url, posted
        FROM jobs
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(jobs)
}
