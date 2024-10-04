pub mod api;
pub mod components;

use crate::{
    database::queries,
    model::{ErrorPage, GameZonePage, HomePage, State, WrongPasswordComponent},
    utils::auth::{encode_jwt, verify_password, Claims},
};
use askama::Template;
use axum::{
    extract::Form,
    http::{header, Response, StatusCode},
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use axum_extra::{headers, TypedHeader};
use cookie::Cookie;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use std::sync::Arc;
use time::Duration;
use tracing::{error, info, warn};

pub async fn home(
    cookies: TypedHeader<headers::Cookie>,
    Extension(state): Extension<Arc<State>>,
) -> impl IntoResponse {
    if let Some(auth_token) = cookies.get("auth_token") {
        match decode::<Claims>(
            auth_token,
            &DecodingKey::from_secret(state.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(token_data) => {
                info!(
                    "Valid session detected for user: {}",
                    token_data.claims.username
                );

                let username = token_data.claims.username;

                let game_zone_template = GameZonePage {
                    first_name: &username,
                };

                Html(game_zone_template.render().unwrap()).into_response()
            }
            Err(_) => {
                error!("Invalid JWT token");

                let error_page_template = ErrorPage;

                Html(error_page_template.render().unwrap()).into_response()
            }
        }
    } else {
        let mut users = queries::fetch_all_users(&state.db).await.unwrap();
        users.reverse();

        let template = HomePage { users };

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

    match queries::fetch_user_by_first_name(&state.db, &first_name).await {
        Ok(user) => {
            if let Ok() = verify_password(&password, &user.password_hash).await
            {
                let jwt = encode_jwt(user.username, &state.jwt_secret)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
                    .unwrap_or_else(|_| "".to_string());

                let cookie = Cookie::build(("auth_token", jwt))
                    .path("/")
                    .max_age(Duration::days(14))
                    .http_only(true);

                info!("User logged in: {}", &first_name);

                Response::builder()
                    .status(StatusCode::FOUND)
                    .header(
                        header::LOCATION,
                        format!("/components/game-zone?user={}", &first_name),
                    )
                    .header(header::SET_COOKIE, cookie.to_string())
                    .body(axum::body::Body::empty())
                    .unwrap()
                    .into_response()
            } else {
                warn!("Wrong password entered for user: {}", &first_name);
                let template = WrongPasswordComponent {
                    first_name: &first_name,
                };
                Html(template.render().unwrap()).into_response()
            }
        }
        Err(_) => {
            error!("User not found: {}", &first_name);
            let template = WrongPasswordComponent {
                first_name: &first_name,
            };
            Html(template.render().unwrap()).into_response()
        }
    }
}

pub async fn logout() -> Redirect {
    Redirect::to("/")
}
