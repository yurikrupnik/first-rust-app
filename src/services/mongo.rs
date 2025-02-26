use mongodb::Client;

pub async fn mongo_connect() -> Client {
    let uri = std::env::var("MONGO_URI").unwrap_or_else(|_| {
        "mongodb+srv://yurikrupnik:T4eXKj1RBI4VnszC@cluster0.rdmew.mongodb.net/".into()
    });

    println!("uri is {}", uri);
    Client::with_uri_str(uri).await.expect("failed to connect!")
}
