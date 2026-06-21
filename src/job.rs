use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct PayloadResponse {
    pub job_id: Uuid,
    pub state: JobStatus,
}

pub async fn create_job(Json(_body): Json<CreateJobRequest>) -> Json<PayloadResponse> {
    let job_id = Uuid::new_v4();
    let response = PayloadResponse {
        job_id,
        state: JobStatus::Queued,
    };
    Json(response)
}
