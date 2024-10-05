use crate::{
    database::queries,
    model::State,
    model::{Game, User},
};
use axum::{routing::get, Extension, Json, Router};
use std::sync::Arc;

pub fn init() -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/games", get(get_games))
}

pub async fn get_users(
    Extension(state): Extension<Arc<State>>,
) -> Json<Vec<User>> {
    let users = queries::fetch_all_users(&state.db).await.unwrap();
    Json(users)
}

pub async fn get_games(
    Extension(state): Extension<Arc<State>>,
) -> Json<Vec<Game>> {
    let games = queries::fetch_all_games(&state.db).await.unwrap();
    Json(games)
}
