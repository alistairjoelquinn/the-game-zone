use std::sync::Arc;

use axum::{routing::get, Extension, Json, Router};

use crate::{database::queries, model::User, state::State};

pub fn init() -> Router {
    Router::new().route("/users", get(get_users))
}

pub async fn get_users(
    Extension(state): Extension<Arc<State>>,
) -> Json<Vec<User>> {
    let users = queries::fetch_all_users(&state.db).await.unwrap();
    Json(users)
}
