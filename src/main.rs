mod model;
mod status;
#[cfg(test)]
mod test;

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::web::{scope};
use model::User;
use mongodb::{bson::doc, Client, Collection};
use dotenv::{dotenv, Error};

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
        .unwrap_or_else(|_|  "mongodb+srv://yurikrupnik:T4eXKj1RBI4VnszC@cluster0.rdmew.mongodb.net/".into());
    
    println!("uri is {}", uri);
    Client::with_uri_str(uri).await.expect("failed to connect")
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

/// Adds a new user to the "users" collection in the database.
#[post("/users")]
async fn add_user(_req: HttpRequest, client: web::Data<Client>) -> HttpResponse {
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
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    mongo_connect().await;
    println!("Connected to mongo");
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
