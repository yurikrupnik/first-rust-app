// use std::net::Ipv4Addr;

mod model;
mod status;
#[cfg(test)]
mod test;
mod services;
mod shared;

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::web::{scope};
use model::User;
use mongodb::{bson::doc, Client, Collection};
use dotenv::{dotenv};
use services::mongo::mongo_connect;

const DB_NAME: &str = "mussia33";
const COLL_NAME: &str = "users";

/// Gets the user with the supplied username.
#[get("/users/{id}")]
async fn get_user(client: web::Data<Client>, id: web::Path<String>) -> HttpResponse {
    let search_id = id.into_inner();
    println!("id is test here {}", search_id);
    let users: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    let data = users.find_one(doc! { "_id": search_id }, None).await;
    if data.is_ok() {
        println!("all good")
    }
    HttpResponse::Ok()
        .content_type("application/json")
        .json("user added")
    // match collection
    //     .find_one(doc! { "_id": &search_id }, None)
    //     // .find(doc! {}, None)
    //     .await
    // {
    //     Ok(Some(user)) => HttpResponse::Ok().json(user),
    //     Ok(None) => {
    //         HttpResponse::NotFound().body(format!("No user found with username {search_id}"))
    //     }
    //     Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    // }
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!!!!")
}

#[get("/stream")]
async fn stream() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json("Hello from yuri!!")
}

/// Adds a new user to the "users" collection in the database.
#[post("/users")]
async fn add_user(_req: HttpRequest, client: web::Data<Client>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection
        .insert_one(
            User {
                name: "test".to_string(),
                age: 12,
                role: "admin".to_string(),
                email: "a@a.com".to_string(),
            },
            None,
        )
        .await;
    match result {
        Ok(_) => HttpResponse::Ok().json("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    mongo_connect().await;
    println!("Connected to mongo!");
    HttpServer::new(move || {
        App::new().service(
            scope("/api")
                .service(stream)
                .service(get_user)
                .service(status::status)
                .service(greet)
                .service(add_user), // .service(status), // .service(postUser)
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
