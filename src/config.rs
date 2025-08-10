use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub mongodb_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: u64,
    pub jwt_refresh_expires_in: u64,
    #[allow(dead_code)]
    pub port: u16,

}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://user:pass@localhost:5432/app".to_string()),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            mongodb_url: env::var("MONGODB_URL")
                .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key".to_string()),
            jwt_expires_in: env::var("JWT_EXPIRES_IN")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()?,
            jwt_refresh_expires_in: env::var("JWT_REFRESH_EXPIRES_IN")
                .unwrap_or_else(|_| "604800".to_string())
                .parse()?,
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
        })
    }
}