mod job;
mod server;
use dotenvy::dotenv;
mod db;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db::init_db().await;

    let app = server::create_router(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("LDS is running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
