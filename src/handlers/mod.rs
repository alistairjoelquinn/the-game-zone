use crate::{
    database::queries,
    model::{
        GameZoneTemplate, HomeTemplate, LoginFieldTemplate, User,
        WrongPasswordTemplate,
    },
    state::State,
    utils::auth::{encode_jwt, Claims},
};
use askama::Template;
use axum::{
    extract::{Form, Query},
    http::{header, Response, StatusCode},
    response::{Html, IntoResponse, Redirect},
    Extension, Json,
};
use cookie::{Cookie, CookieJar};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::Duration;
use tracing::info;

pub async fn get_users(
    Extension(state): Extension<Arc<State>>,
) -> Json<Vec<User>> {
    let users = queries::fetch_all_users(&state.db).await.unwrap();
    Json(users)
}

pub async fn home(
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    let jar = CookieJar::new();
    if let Some(auth_token) = jar.get("auth_token").map(|c| c.value()) {
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
        let mut users = queries::fetch_all_users(&state.db).await.unwrap();
        users.reverse();

        let template = HomeTemplate { users };

        Html(template.render().unwrap()).into_response()
    }
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

#[derive(Deserialize, Serialize, Debug)]
pub struct GameZoneQuery {
    user: String,
}

pub async fn game_zone(
    Query(params): Query<GameZoneQuery>,
) -> impl IntoResponse {
    let template = GameZoneTemplate {
        first_name: &params.user,
    };

    Html(template.render().unwrap()).into_response()
}
