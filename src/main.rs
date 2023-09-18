#![allow(unused)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
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

async fn create_task(task: web::Json<Task>) -> impl Responder {
    let new_task = task.into_inner();

    HttpResponse::Created().json(new_task)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/task", web::get().to(read_task))
            .route("/create_task", web::post().to(create_task))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
