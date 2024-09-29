pub mod api;
pub mod components;

use crate::{
    database::queries,
    model::{GameZoneTemplate, HomeTemplate, WrongPasswordTemplate},
    state::State,
    utils::auth::{encode_jwt, Claims},
};
use askama::Template;
use axum::{
    extract::{Form, Query},
    http::{header, Response, StatusCode},
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use axum_extra::{headers, TypedHeader};
use cookie::Cookie;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::Duration;
use tracing::info;

pub async fn home(
    cookies: TypedHeader<headers::Cookie>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    println!("Cookies: {:?}", cookies);
    if let Some(auth_token) = cookies.get("auth_token") {
        println!("Auth token: {}", auth_token);
        match decode::<Claims>(
            auth_token,
            &DecodingKey::from_secret(state.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(token_data) => {
                println!("Token data: {:?}", token_data);
                let template = GameZoneTemplate {
                    first_name: "johner",
                };

                Html(template.render().unwrap()).into_response()
            }
            Err(_) => {
                // Invalid JWT, redirect to login
                Redirect::to("/login").into_response()
            }
        }
    } else {
        println!("No auth token found");
        let mut users = queries::fetch_all_users(&state.db).await.unwrap();
        users.reverse();

        let template = HomeTemplate { users };

        Html(template.render().unwrap()).into_response()
    }
}

#[derive(Deserialize, Debug)]
pub struct LoginBody {
    pub first_name: String,
    pub password: String,
}

pub async fn login(
    Extension(state): Extension<Arc<State>>,
    Form(form): Form<LoginBody>,
) -> impl IntoResponse {
    let first_name = form.first_name;
    let password = form.password;

    info!(
        "Login attempt - First Name: {}, Password: {}",
        first_name, password
    );

    match queries::fetch_user_by_first_name(&state.db, &first_name).await {
        Ok(user) => {
            if user.password == password {
                let jwt = encode_jwt(user.username, &state.jwt_secret)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
                    .unwrap_or_else(|_| "".to_string());

                let cookie = Cookie::build(("auth_token", jwt))
                    .path("/")
                    .max_age(Duration::days(14))
                    .http_only(true);

                Response::builder()
                    .status(StatusCode::FOUND)
                    .header(
                        header::LOCATION,
                        format!("/game-zone?user={}", &first_name),
                    )
                    .header(header::SET_COOKIE, cookie.to_string())
                    .body(axum::body::Body::empty())
                    .unwrap()
                    .into_response()
            } else {
                let template = WrongPasswordTemplate {
                    first_name: &first_name,
                };
                Html(template.render().unwrap()).into_response()
            }
        }
        Err(_) => {
            let template = WrongPasswordTemplate {
                first_name: &first_name,
            };
            Html(template.render().unwrap()).into_response()
        }
    }
}

pub async fn logout() -> Redirect {
    Redirect::to("/")
}
