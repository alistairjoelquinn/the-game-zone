use askama_axum::Template;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub users: Vec<User>,
}

#[derive(Template)]
#[template(path = "login-field.html")]
pub struct LoginFieldTemplate<'a> {
    pub first_name: &'a str,
}

#[derive(Template)]
#[template(path = "wrong-password.html")]
pub struct WrongPasswordTemplate<'a> {
    pub first_name: &'a str,
}

#[derive(Template)]
#[template(path = "main.html")]
pub struct GameZoneTemplate<'a> {
    pub first_name: &'a str,
}
