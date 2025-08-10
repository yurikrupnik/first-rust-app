use mongodb::Client;
use tracing::info;

pub async fn connect(mongodb_url: &str) -> anyhow::Result<Client> {
    info!("Connecting to MongoDB...");
    
    let client = Client::with_uri_str(mongodb_url).await?;
    
    client
        .database("admin")
        .run_command(mongodb::bson::doc! {"ping": 1})
        .await?;
    
    info!("Connected to MongoDB");
    Ok(client)
}