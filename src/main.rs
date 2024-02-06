use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::Value;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

#[derive(Debug, Serialize)]
struct Person {
    title: String,
    name: String,
    marketing: bool,
}

static DB: Lazy<Surreal<Client>> = Lazy::new(|| Surreal::init());
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Connecting to the database...");
    DB.connect::<Ws>("127.0.0.1:8000")
        .await
        .expect("Unnable to connect to the database");
    println!("Connected!");

    println!("Signing in to the database...");
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("Unnable to sign in to the database");
    println!("Signed in!");

    println!("Setting up the namespace and database...");

    DB.use_ns("test")
        .use_db("test")
        .await
        .expect("Unnable to select namespace/database");
    println!("Setup complete!");

    println!("Starting Actix server on http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(insert_person)
            .service(query_person)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/insert_person")]
async fn insert_person() -> impl Responder {
    let person = Person {
        title: "Founder & CEO".to_string(),
        name: "Rootster".to_string(),
        marketing: true,
    };

    let result: Result<Vec<Value>, Error> = DB.create("person").content(&person).await;

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error inserting person: {}", e))
        }
    }
}

#[get("/query_person")]
async fn query_person() -> impl Responder {
    let result: Result<Vec<Value>, Error> = DB.select("person").await;

    match result {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error querying person: {}", e)),
    }
}
