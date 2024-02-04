use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

#[derive(Debug, Serialize)]
struct Person {
    title: String,
    name: String,
    marketing: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Actix server on http://127.0.0.1:8080");
    HttpServer::new(|| {
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
    let db = match Surreal::new::<Ws>("0.0.0.0:8000").await {
        Ok(db) => db,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error connecting to database: {}", e))
        }
    };

    if let Err(e) = db
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
    {
        return HttpResponse::InternalServerError().body(format!("Error signing in: {}", e));
    }

    if let Err(e) = db.use_ns("test").use_db("test").await {
        return HttpResponse::InternalServerError()
            .body(format!("Error selecting namespace/database: {}", e));
    }

    let person = Person {
        title: "Founder & CEO".to_string(),
        name: "Rootster".to_string(),
        marketing: true,
    };

    let result: Result<Vec<Value>, Error> = db.create("person").content(&person).await;

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error inserting person: {}", e))
        }
    }
}

#[get("/query_person")]
async fn query_person() -> impl Responder {
    let db = match Surreal::new::<Ws>("0.0.0.0:8000").await {
        Ok(db) => db,
        Err(e) => return format!("Error connecting to database: {}", e),
    };

    if let Err(e) = db
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
    {
        return format!("Error signing in: {}", e);
    }

    if let Err(e) = db.use_ns("test").use_db("test").await {
        return format!("Error selecting namespace/database: {}", e);
    }

    let result: Result<Vec<Value>, Error> = db.select("person").await;
    match result {
        Ok(records) => format!("{}", serde_json::json!(records)),
        Err(e) => format!("Error querying person: {}", e),
    }
}
