use crate::job::Job;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn claim_job(pool: &PgPool) -> Option<Job> {
    sqlx::query_as::<_, Job>(
        "UPDATE jobs
    SET state = 'Running'
    WHERE id = (
    SELECT id FROM jobs
    WHERE state = 'Queued'
    ORDER BY created_at ASC
    LIMIT 1
    FOR UPDATE SKIP LOCKED
    )
    RETURNING id, job_type, payload, state, attempts",
    )
    .fetch_optional(pool)
    .await
    .unwrap()
}

pub async fn execute_job(job: &Job) -> Result<(), String> {
    match job.job_type.as_str() {
        "send_email" => {
            println!("Sending email to: {}", job.payload["to"]);
            Ok(())
        }
        "resize_image" => {
            println!("Resizing Image : {}", job.payload["url"]);
            Ok(())
        }
        "send_sms" => {
            println!("Sending sms to: {}", job.payload["to"]);
            Ok(())
        }
        "generate_report" => {
            println!("Generating report for: {}", job.payload["user_id"]);
            Ok(())
        }
        "process_payment" => {
            println!("Process payment of: {}", job.payload["amount"]);
            Ok(())
        }
        "slow_job" => {
            let duration = job.payload["duration_ms"]
                .as_u64()
                .unwrap_or(1000);
            
            tokio::time::sleep(tokio::time::Duration::from_millis(duration)).await;
            println!("slow job done after {}ms", job.id);
            Ok(())
        }
        _ => Err(format!("Uknown Job type: {}", job.job_type)),
    }
}

async fn complete_job(pool: &PgPool, job_id: Uuid, state: &str) {
    sqlx::query("UPDATE jobs SET state = $1 WHERE id = $2")
        .bind(state)
        .bind(job_id)
        .execute(pool)
        .await
        .unwrap();
}

async fn worker_loop(pool: PgPool) {
    loop {
        match claim_job(&pool).await {
            Some(job) => match execute_job(&job).await {
                Ok(_) => complete_job(&pool, job.id, "Succeeded").await,
                Err(e) => {
                    println!("Job Failed: {}", e);
                    complete_job(&pool, job.id, "Failed").await;
                }
            },
            None => {
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        }
    }
}

pub fn run_workers(pool: PgPool, count: u32) {
    for i in 0..count {
        let pool = pool.clone();
        tokio::spawn(async move {
            println!("worker {} started", i);
            worker_loop(pool).await;
        });
    }
}
