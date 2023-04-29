#[warn(unused_imports)]
// Example code for using MongoDB with Actix.

// struct EnvVars {
//     mongo_uri: str,
// }
mod model;
mod status;
#[cfg(test)]
mod test;

// use status::;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Scope};
// use std::thread::scope;
// use actix_web::body::None;
use actix_web::web::{scope, Json};
use model::User;
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, Cursor, IndexModel};
// use serde::Deserialize;
use dotenv::dotenv;

const DB_NAME: &str = "test";
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
    // data.and_then(fn ds(d: any) {
    //     println!(d)
    // });
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

async fn mongo_connect() -> Client {
    let uri = std::env::var("MONGODB_URI")
        // .and_then(|_| "mongodb://localhost/first-rust-app".into())
        .unwrap_or_else(|_|  "mongodb+srv://yurikrupnik:T4eXKj1RBI4VnszC@cluster0.rdmew.mongodb.net/".into());
    
    // uri.chars();
    println!("uri is {}", uri);
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    client
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/stream")]
async fn stream() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json("Hello there")
}

/// Gets the users array.
// #[get("/users")]
// async fn get_users(client: web::Data<Client>) -> impl Responder {
//     let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
//
//     // let mut cursor = collection.find(None, None).await;
//     // cursor.into_ok();
//     // Ok(cursor);
//     // let data = cursor.expect();
//     // while let Some(user) = cursor.try_next().await {
//     //     println!("user: {}", user.name)
//     // }
//     match collection
//         // .find_one(doc! { "username": &username }, None)
//         .find(doc! {}, None)
//         .await
//     {
//         // Ok(Some(user)) => HttpResponse::Ok().json(user),
//         Ok((users)) => HttpResponse::Ok().json(users),
//         // Ok(Some(users)) => HttpResponse::Ok().json(users),
//         // Ok(None) => HttpResponse::NotFound().body(format!("No users found")),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
//     // let mut my_vector: Vec<User> = Vec::new();
//     // let user = User {
//     //     email: "a@c.com".to_string(),
//     //     age: 123,
//     //     // _id: "Adsdassafsdfdfgfdbhbfghgfdgfdg".to_string(),
//     //     name: "Ars".into(),
//     //     role: "admin".to_string(),
//     // };
//     // println!("age as is {}", user.age());
//     // my_vector.push(user);
//     //
//     // // println!("first items name: {}", my_vector[0].name);
//     // // let my_vector: Vec<User> = vec![];
//     // // return Json(my_vector);
//     // HttpResponse::Ok()
//     //     .content_type("application/json")
//     //     .json(my_vector)
// }

/// Adds a new user to the "users" collection in the database.
#[post("/users")]
async fn add_user(req: HttpRequest, client: web::Data<Client>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    // println!("email {}", form.email);
    // println!("body {}", body.email);
    // println!("req {}", req.uri());
    // println!("query_string {}", req.query_string());
    // println!("method {}", req.method());
    // HttpResponse::Ok()
    //     .content_type("application/json")
    //     .json("user added")
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
// #[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost/first-rust-app".into());
    // let client = Client::with_uri_str(uri).await.expect("failed to connect");
    mongo_connect().await;
    println!("Connected to mongo");
    // create_username_index(&client).await;
    HttpServer::new(move || {
        App::new().service(
            scope("/api")
                .service(stream)
                .service(get_user)
                // .service(get_users)
                // .service(get_user)
                .service(status::status)
                .service(add_user), // .service(status), // .service(postUser)
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
    // .expect("all good")?
}
