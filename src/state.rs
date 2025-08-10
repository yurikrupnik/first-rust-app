use sqlx::{Pool, Postgres};
use redis::Client as RedisClient;
use mongodb::Client as MongoClient;

use crate::{config::Config, database};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: Pool<Postgres>,
    #[allow(dead_code)]
    pub redis: RedisClient,
    #[allow(dead_code)]
    pub mongo: MongoClient,
}

impl AppState {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let (db, redis, mongo) = tokio::try_join!(
            database::postgres::connect(&config.database_url),
            async { database::redis::connect(&config.redis_url) },
            database::mongodb::connect(&config.mongodb_url)
        )?;

        Ok(Self {
            config,
            db,
            redis,
            mongo,
        })
    }
}