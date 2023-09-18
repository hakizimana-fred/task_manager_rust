#![allow(unused)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

lazy_static::lazy_static! {
    static ref TASKS: Arc<Mutex<Vec<Task>>> = Arc::new(Mutex::new(Vec::new()));
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

// Respond with JSON
async fn read_task(path: web::Path<u32>) -> impl Responder {
    let task_id = path.into_inner();
    let tasks = TASKS.lock().unwrap();
    HttpResponse::Ok().body(task_id.to_string())
}

async fn create_task(task: web::Json<Task>) -> impl Responder {
    let new_task = task.into_inner();
    let new_task_clone = new_task.clone();
    let mut tasks = TASKS.lock().unwrap();
    tasks.push(new_task);
    HttpResponse::Created().json(new_task_clone)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/task/{id}", web::get().to(read_task))
            .route("/create_task", web::post().to(create_task))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
