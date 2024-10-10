use crate::aws::s3::S3Client;
use askama_axum::Template;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct State {
    pub db: sqlx::PgPool,
    pub s3: Arc<S3Client>,
    pub jwt_secret: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Game {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub slug: String,
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
#[template(path = "components/went-wrong.html")]
pub struct WentWrongComponent<'a> {
    pub first_name: &'a str,
}

#[derive(Template)]
#[template(path = "components/game-zone.html")]
pub struct GameZoneComponent<'a> {
    pub first_name: &'a str,
}

#[derive(Template)]
#[template(path = "components/game.html")]
pub struct GameComponent<'a> {
    pub slug: &'a str,
}

#[derive(Template)]
#[template(path = "game-zone-page.html")]
pub struct GameZonePage<'a> {
    pub first_name: &'a str,
    pub games: Vec<Game>,
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct ErrorPage;
