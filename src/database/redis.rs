use redis::Client;
use tracing::info;

pub fn connect(redis_url: &str) -> anyhow::Result<Client> {
    info!("Connecting to Redis...");
    
    let client = Client::open(redis_url)?;
    
    info!("Connected to Redis");
    Ok(client)
}