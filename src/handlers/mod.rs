use askama::Template;
use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    database::queries::{
        create_user, delete_user_by_id, fetch_all_users, fetch_user_by_id,
        update_user,
    },
    model::{HomeTemplate, LoginFieldTemplate, User},
    state::State,
};

#[derive(Deserialize, Serialize)]
pub struct UserId {
    id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Username {
    name: String,
}

pub async fn get_user() -> &'static str {
    "GET /user"
}

pub async fn post_user(
    Extension(state): Extension<Arc<State>>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    let response = match create_user(&state.db, user).await {
        Ok(_) => "New user added",
        Err(_) => "Failed to add new user",
    };
    IntoResponse::into_response(response)
}

pub async fn get_user_by_id(
    Path(UserId { id }): Path<UserId>,
    Extension(state): Extension<Arc<State>>,
) -> Json<User> {
    let user = fetch_user_by_id(&state.db, id).await.unwrap();
    Json(user)
}

pub async fn patch_user(
    Path(UserId { id }): Path<UserId>,
    Extension(state): Extension<Arc<State>>,
    Json(user): Json<User>,
) -> Json<User> {
    let user = update_user(&state.db, user, id).await.unwrap();
    Json(user)
}

pub async fn delete_user(
    Path(UserId { id }): Path<UserId>,
    Extension(state): Extension<Arc<State>>,
) -> Json<User> {
    let user = delete_user_by_id(&state.db, id).await.unwrap();
    Json(user)
}

pub async fn get_users(
    Extension(state): Extension<Arc<State>>,
) -> Json<Vec<User>> {
    let users = fetch_all_users(&state.db).await.unwrap();
    Json(users)
}

pub async fn home(Extension(state): Extension<Arc<State>>) -> Html<String> {
    let mut users = fetch_all_users(&state.db).await.unwrap();
    users.reverse();

    let template = HomeTemplate {
        title: "Koen & Jonah's Game Zone",
        users,
    };

    Html(template.render().unwrap())
}

pub async fn login_field(
    Path(Username { name }): Path<Username>,
) -> Html<String> {
    println!("Name: {}", name);
    let template = LoginFieldTemplate { first_name: &name };

    Html(template.render().unwrap())
}
