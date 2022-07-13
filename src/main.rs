//! Example code for using MongoDB with Actix.

// struct EnvVars {
//     mongo_uri: str,
// }

mod model;
#[cfg(test)]
mod test;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Scope};
// use std::thread::scope;
use actix_web::body::None;
use actix_web::web::scope;
use model::User;
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, Cursor, IndexModel};
use serde::Deserialize;

const DB_NAME: &str = "test";
const COLL_NAME: &str = "users";

#[derive(Debug, Deserialize)]
pub enum ResponseType {
    Token,
    Code,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    id: u64,
    response_type: ResponseType,
}

/// Gets the user with the supplied username.
#[get("/users/{id}")]
async fn get_user(client: web::Data<Client>, id: web::Path<String>) -> HttpResponse {
    let search_id = id.into_inner();
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    match collection
        .find_one(doc! { "_id": &search_id }, None)
        // .find(doc! {}, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username {search_id}"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }
/// Creates an index on the "username" field to force the values to be unique.
// async fn create_username_index(client: &Client) {
//     let options = IndexOptions::builder().unique(true).build();
//     let model = IndexModel::builder()
//         .keys(doc! { "username": 1 })
//         .options(options)
//         .build();
//     client
//         .database("users")
//         .collection::<User>("users")
//         .create_index(model, None)
//         .await
//         .expect("creating an index should succeed");
// }

async fn mongo_connect() -> Client {
    let uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost/first-rust-app".into());
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    client
}

// #[get("/hello/{name}")]
// async fn greet(name: web::Path<String>) -> impl Responder {
//     format!("Hello {name}!")
// }

#[get("/status")]
async fn status() -> impl Responder {
    "{\"status\": \"up\"}"
}

#[get("/stream")]
async fn stream() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json("hello")
}

/// Gets the users array.
// #[get("/users")]
// async fn get_users(client: web::Data<Client>, username: web::Path<String>) -> HttpResponse {
//     // async fn get_users() -> HttpResponse {
//     let clone = client.clone();
//     // HttpResponse::Ok().json("val")
//     // println!("my clone name {}", clone.database("test"))
//     // HttpResponse::Ok()
//     //     .content_type("application/json")
//     //     .json("hello")
//     let username = username.into_inner();
//     let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
//     match collection
//         // .find_one(doc! { "username": &username }, None)
//         .find(doc! {}, None)
//         .await
//     {
//         Ok((users)) => HttpResponse::Ok().json(users),
//         // Ok(Some(users)) => HttpResponse::Ok().json(users),
//         Ok(None) => HttpResponse::NotFound().body(format!("No users found")),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

/// Adds a new user to the "users" collection in the database.
#[post("/users")]
async fn add_user(client: web::Data<Client>, form: web::Form<User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    println!("email {}", form.email);
    let result = collection.insert_one(form.into_inner(), None).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// #[tokio::main]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost/first-rust-app".into());
    // let client = Client::with_uri_str(uri).await.expect("failed to connect");
    let client = mongo_connect();
    // create_username_index(&client).await;

    HttpServer::new(move || {
        App::new().service(
            scope("/api")
                .service(stream)
                // .service(get_users)
                .service(get_user)
                .service(add_user)
                .service(status), // .service(postUser)
        )
        // .app_data(web::Data::new(client.clone()))
        // .route("/users", web::get().to(|| async { "Hello World!" }))
        // .service(add_user)
        // .service(get_user)
        // .service(get_users)
        // .route("/echo", web::get().to(|| async { "Hello World!" }))
        // .service(echo)
        // .route("/hello", web::get().to(|| async { "Hello World!" }))
        // .route("/", web::get().to(status))
        // .route("/stream", web::get().to(stream))
        // .route("/", web::get().to(status))
        // .route("/hello/{name}", web::get().to(greet))
        // .service(stream);
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
