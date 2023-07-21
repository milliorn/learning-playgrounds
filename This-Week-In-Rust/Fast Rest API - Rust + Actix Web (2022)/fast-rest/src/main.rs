use actix_web::{get, web, App, HttpServer}; /* https://actix.rs/ */
use serde::{Deserialize, Serialize}; /* used for json */
use std::sync::Mutex; /* enables locking memory */

mod todolist;
use todolist::services;

/* shared state to pass to each route handler */
struct AppState {
    todolist_entries: Mutex<Vec<TodolistEntry>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TodolistEntry {
    id: i32,
    date: i64,
    title: String
}

#[get("/")]
async fn index() -> String {
    "Hello World!".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todolist_entries: Mutex::new(vec![])
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
