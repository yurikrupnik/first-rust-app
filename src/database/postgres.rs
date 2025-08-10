use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;

pub async fn connect(database_url: &str) -> anyhow::Result<Pool<Postgres>> {
    info!("Connecting to PostgreSQL...");
    
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(10))
        .idle_timeout(std::time::Duration::from_secs(30))
        .connect(database_url)
        .await?;

    info!("Connected to PostgreSQL");
    // info!("Connected to PostgreSQL {:?}", pool.options());
    Ok(pool)
}

#[allow(dead_code)]
pub async fn migrate(pool: &Pool<Postgres>) -> anyhow::Result<()> {
    info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(pool).await?;
    info!("Database migrations completed");
    Ok(())
}