#![allow(unused)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

struct Task {
    id: u32,
    title: String,
    completed: bool,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

async fn read_task() -> impl Responder {
    let data = json!({
        "message": "well done reading a book",
    });

    // Respond with JSON
    HttpResponse::Ok().json(data)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/task", web::get().to(read_task))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
