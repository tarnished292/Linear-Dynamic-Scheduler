mod db;
mod job;
mod server;
mod worker;
use dotenvy::dotenv;

use crate::worker::run_workers;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db::init_db().await;

    let worker_count = 10;

    run_workers(pool.clone(), worker_count);

    let app = server::create_router(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("LDS is running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
