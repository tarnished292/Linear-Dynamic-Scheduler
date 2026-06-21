use axum::{Router, routing::{post}};
use crate::job::create_job;
use sqlx::{PgPool};

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/job", post(create_job))
        .with_state(pool)
}



#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}