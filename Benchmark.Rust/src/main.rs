use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use uuid::Uuid;

mod models;
use crate::models::user::*;

#[get("/api/user")]
async fn get_user() -> impl Responder {
    let user = User::new(Uuid::new_v4(), "Benjamin", "HEINTZ", "heintz.benjamin@gmaail.com");
    HttpResponse::Ok().json(user)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    
    let port = 5600;
    println!("\n Server running at port : {}", port);

    HttpServer::new(|| {
        App::new().service(get_user)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
