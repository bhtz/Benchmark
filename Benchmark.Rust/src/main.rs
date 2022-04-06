#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use uuid::Uuid;
#[path = "models/User.rs"] mod models;

#[get("/user")]
fn get_user() -> Json<models::User> {
    let uuid_str = Uuid::new_v4().to_hyphenated().to_string();
    let user = models::build_user(uuid_str, String::from("Benjamin"), String::from("HEINTZ"), String::from("heintz.benjamin@gmaail.com"));
    Json(user)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![get_user])
}