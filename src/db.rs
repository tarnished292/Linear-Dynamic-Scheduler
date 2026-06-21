use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn init_db() -> PgPool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS jobs (
            id          UUID PRIMARY KEY,
            job_type    TEXT NOT NULL,
            payload     JSONB NOT NULL,
            state      TEXT NOT NULL,
            attempts    INTEGER DEFAULT 0,
            created_at  TIMESTAMPTZ DEFAULT NOW()
        )"
    )
    .execute(&pool)
    .await  
    .unwrap();

    println!("DB connected");
    pool
}