use axum::{Json, extract::State};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::PgPool;

#[derive(Serialize)]
pub enum JobStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
    Retrying,
    Expired,
}

#[derive(Deserialize)]
pub struct CreateJobRequest {
    pub job_type: String,
    pub payload: serde_json::Value,
}

#[derive(Serialize)]
pub struct PayloadResponse {
    pub job_id: Uuid,
    pub state: JobStatus,
}

pub async fn create_job(State(pool): State<PgPool>, Json(body): Json<CreateJobRequest>) -> Json<PayloadResponse> {
    let job_id = Uuid::new_v4();
    let response = PayloadResponse {
        job_id,
        state: JobStatus::Queued,
    };

    sqlx::query("INSERT INTO jobs (id, job_type, payload) VALUES ($1, $2, $3)")
        .bind(job_id)
        .bind(&body.job_type)
        .bind(&body.payload)
        .execute(&pool)
        .await
        .unwrap();

    Json(response)
}
