use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use serde_json::Value;
use std::sync::Mutex;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

#[derive(Debug, Serialize)]
struct Person {
    title: String,
    name: String,
    marketing: bool,
}

struct SurrealData {
    db: Mutex<Surreal<Client>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Actix server on http://127.0.0.1:8080");
    let db = Surreal::new::<Ws>("0.0.0.0:8000")
        .await
        .expect("Error connecting to database");
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("Error signing in");
    let db = web::Data::new(SurrealData { db: Mutex::new(db) });

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
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
async fn insert_person(db: web::Data<SurrealData>) -> impl Responder {
    let db_lock = match db.db.lock() {
        Ok(lock) => lock,
        Err(_) => {
            return HttpResponse::ServiceUnavailable()
                .body("Service is temporarily unavailable. Please try again later.")
        }
    };

    if let Err(e) = db_lock.use_ns("test").use_db("test").await {
        return HttpResponse::InternalServerError()
            .body(format!("Error selecting namespace/database: {}", e));
    }

    let person = Person {
        title: "Founder & CEO".to_string(),
        name: "Rootster".to_string(),
        marketing: true,
    };

    let result: Result<Vec<Value>, Error> = db_lock.create("person").content(&person).await;

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error inserting person: {}", e))
        }
    }
}

#[get("/query_person")]
async fn query_person(db: web::Data<SurrealData>) -> impl Responder {
    let db_lock = match db.db.lock() {
        Ok(lock) => lock,
        Err(_) => {
            return HttpResponse::ServiceUnavailable()
                .body("Service is temporarily unavailable. Please try again later.")
        }
    };

    if let Err(e) = db_lock.use_ns("test").use_db("test").await {
        return HttpResponse::InternalServerError()
            .body(format!("Error selecting namespace/database: {}", e));
    }

    let result: Result<Vec<Value>, Error> = db_lock.select("person").await;

    match result {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error querying person: {}", e)),
    }
}
