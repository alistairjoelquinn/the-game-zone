use askama_axum::{IntoResponse, Template};
use axum::{extract::Query, response::Html, routing::get, Router};
use serde::{Deserialize, Serialize};

use crate::model::{GameZoneTemplate, LoginFieldTemplate};

pub fn init() -> Router {
    Router::new()
        .route("/login-field", get(login_field))
        .route("/game-zone", get(game_zone)) //  .layer(middleware::from_fn(auth::authorize)),
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
