use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    id: String,
    firstname: String,
    lastname: String,
    email: String
}

pub fn build_user (id: String, firstname: String, lastname: String, email: String) -> User {
    User {
        id,
        firstname,
        lastname,
        email
    }
}