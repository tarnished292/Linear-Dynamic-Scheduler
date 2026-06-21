use axum::{Router, routing::{post, get}};
use crate::job::{create_job, get_job};
use sqlx::{PgPool};

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/job", post(create_job))
        .route("/job/{id}", get(get_job))
        .with_state(pool)
}
