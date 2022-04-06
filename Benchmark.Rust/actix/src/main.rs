use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use uuid::Uuid;

mod models;
use crate::models::user::*;

#[get("/api/user")]
async fn get_user() -> impl Responder {
    let uuid_str = Uuid::new_v4().to_hyphenated().to_string();
    let user = build_user(uuid_str, String::from("Benjamin"), String::from("HEINTZ"), String::from("heintz.benjamin@gmaail.com"));
    HttpResponse::Ok().json(user)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_user)
    })
    .bind(("127.0.0.1", 5600))?
    .run()
    .await
}