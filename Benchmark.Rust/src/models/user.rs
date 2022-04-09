use serde::{Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    id: Uuid,
    firstname: String,
    lastname: String,
    email: String
}

impl User {
    pub fn new (id: Uuid, firstname: &str, lastname: &str, email: &str) -> User {
        User {
            id, 
            firstname: firstname.to_string(), 
            lastname: lastname.to_string(), 
            email: email.to_string()
        }
    }
}
