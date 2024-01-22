use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use uuid::Uuid;
use std::collections::HashMap;

mod models;
use crate::models::user::*;

#[get("/")]
async fn get_msg() -> impl Responder {
    let mut msg = HashMap::new();
    msg.insert("name".to_string(), "Benchmark.Rust".to_string());
    HttpResponse::Ok().json(&msg)
}

#[get("/api/user")]
async fn get_user() -> impl Responder {
    let user = User::new(Uuid::new_v4(), "Benjamin", "HEINTZ", "heintz.benjamin@gmail.com");
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let port = 8080;
    println!("\n Server running at port : {}", port);

    HttpServer::new(|| {
        App::new()
        .service(get_msg)
        .service(get_user)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
