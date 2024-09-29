use askama_axum::Template;
use axum::{extract::Query, response::Html, routing::get, Router};
use serde::{Deserialize, Serialize};

use crate::model::LoginFieldTemplate;

pub fn init() -> Router {
    Router::new()
        .route("/login-field", get(login_field))
        .route("/wrong-password", get(wrong_password))
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
