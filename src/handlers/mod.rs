use askama::Template;
use axum::{
    extract::{Form, Path, Query},
    response::{Html, IntoResponse, Redirect},
    Extension, Json,
};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    database::queries::{
        create_user, delete_user_by_id, fetch_all_users,
        fetch_user_by_first_name, fetch_user_by_id, update_user,
    },
    model::{HomeTemplate, LoginFieldTemplate, User, WrongPasswordTemplate},
    state::State,
};

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

#[derive(Deserialize, Serialize)]
pub struct UserId {
    id: i32,
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

#[derive(Deserialize, Serialize)]
pub struct LoginFieldQuery {
    user: String,
}

pub async fn login_field(
    Query(params): Query<LoginFieldQuery>,
) -> Html<String> {
    let name = params.user;
    let template = LoginFieldTemplate { first_name: &name };

    Html(template.render().unwrap())
}

#[derive(Deserialize, Debug)]
pub struct LoginBody {
    pub first_name: String,
    pub password: String,
}

#[debug_handler]
pub async fn login(
    Extension(state): Extension<Arc<State>>,
    Form(form): Form<LoginBody>,
) -> impl IntoResponse {
    println!("Form: {:?}", form);
    let first_name = form.first_name;
    let password = form.password;

    match fetch_user_by_first_name(&state.db, &first_name).await {
        Ok(user) => {
            if user.password == password {
                Redirect::to("/game-zone").into_response()
            } else {
                let template = WrongPasswordTemplate {
                    first_name: &first_name,
                };
                Html(template.render().unwrap()).into_response()
            }
        }
        Err(_) => {
            // Handle the case where the user is not found
            // For simplicity, we'll return the same template as wrong password
            let template = WrongPasswordTemplate {
                first_name: &first_name,
            };
            Html(template.render().unwrap()).into_response()
        }
    }
}
