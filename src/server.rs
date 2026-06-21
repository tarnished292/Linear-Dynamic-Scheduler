use axum::{Router, routing::{post}};
use crate::job::create_job;

pub fn create_router() -> Router {
    Router::new()
        .route("/job", post(create_job))
}