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
#[template(path = "home-page.html")]
pub struct HomePage {
    pub users: Vec<User>,
}

#[derive(Template)]
#[template(path = "components/login-field.html")]
pub struct LoginFieldComponents<'a> {
    pub first_name: &'a str,
}

#[derive(Template)]
#[template(path = "components/wrong-password.html")]
pub struct WrongPasswordComponent<'a> {
    pub first_name: &'a str,
}

#[derive(Template)]
#[template(path = "components/game-zone.html")]
pub struct GameZoneComponent<'a> {
    pub first_name: &'a str,
}

#[derive(Template)]
#[template(path = "game-zone-page.html")]
pub struct GameZonePage<'a> {
    pub first_name: &'a str,
}
