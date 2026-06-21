use axum::{
    Json,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize)]
pub enum JobStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
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

pub async fn create_job(
    State(pool): State<PgPool>,
    Json(body): Json<CreateJobRequest>,
) -> Json<PayloadResponse> {
    let job_id = Uuid::new_v4();
    let response = PayloadResponse {
        job_id,
        state: JobStatus::Queued,
    };

    sqlx::query("INSERT INTO jobs (id, job_type, payload, state) VALUES ($1, $2, $3, $4)")
        .bind(job_id)
        .bind(&body.job_type)
        .bind(&body.payload)
        .bind("Queued")
        .execute(&pool)
        .await
        .unwrap();

    Json(response)
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Job {
    pub id: Uuid,
    pub job_type: String,
    pub payload: serde_json::Value,
    pub attempts: i32,
    pub state: String,
}

pub async fn get_job(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Json<Job> {
    let job = sqlx::query_as::<_, Job>(
        "SELECT id, job_type, payload, state, attempts FROM jobs WHERE id=$1",
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(job)
}
