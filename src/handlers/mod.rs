pub mod api;
pub mod components;

use crate::{
    database::queries,
    model::{ErrorPage, GameZonePage, HomePage, State, WentWrongComponent},
    utils::auth::{encode_jwt, Claims},
};
use askama::Template;
use axum::{
    body::Body,
    extract::Form,
    http::{header, Response, StatusCode},
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use axum_extra::{headers, TypedHeader};
use bcrypt::verify;
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
                let games = queries::fetch_all_games(&state.db).await.unwrap();

                let game_zone_template = GameZonePage {
                    first_name: &username,
                    games,
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

    let user =
        match queries::fetch_user_by_first_name(&state.db, &first_name).await {
            Ok(user) => user,
            Err(_) => {
                error!("User not found: {}", &first_name);
                return render_went_wrong(&first_name);
            }
        };

    let is_valid = match verify(&password, &user.password_hash) {
        Ok(valid) => valid,
        Err(_) => {
            error!("Error checking password for user: {}", &first_name);
            return render_went_wrong(&first_name);
        }
    };

    if !is_valid {
        warn!("Wrong password entered for user: {}", &first_name);
        return render_went_wrong(&first_name);
    }

    let jwt = match encode_jwt(user.username, &state.jwt_secret) {
        Ok(token) => token,
        Err(_) => {
            error!("Failed to generate JWT for user: {}", &first_name);
            return render_went_wrong(&first_name);
        }
    };

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
}

fn render_went_wrong(first_name: &str) -> Response<Body> {
    let template = WentWrongComponent { first_name };
    Html(template.render().unwrap()).into_response()
}

pub async fn logout() -> Redirect {
    Redirect::to("/")
}
