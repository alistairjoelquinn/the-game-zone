use crate::{
    database::queries,
    model::{
        GameZoneTemplate, HomeTemplate, LoginFieldTemplate, User,
        WrongPasswordTemplate,
    },
    state::State,
    utils::auth::is_valid_token,
};
use askama::Template;
use axum::{
    extract::{Form, Query},
    response::{Html, IntoResponse, Redirect},
    Extension, Json,
};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub async fn get_users(
    Extension(state): Extension<Arc<State>>,
) -> Json<Vec<User>> {
    let users = queries::fetch_all_users(&state.db).await.unwrap();
    Json(users)
}

pub async fn home(Extension(state): Extension<Arc<State>>) -> Html<String> {
    let mut users = queries::fetch_all_users(&state.db).await.unwrap();
    users.reverse();

    let template = HomeTemplate { users };

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

pub async fn login(
    Extension(state): Extension<Arc<State>>,
    Form(form): Form<LoginBody>,
) -> impl IntoResponse {
    let first_name = form.first_name;
    let password = form.password;

    match queries::fetch_user_by_first_name(&state.db, &first_name).await {
        Ok(user) => {
            if user.password == password {
                // let token = encode_jwt(user.username)?;

                // Ok(Json(token));
                Redirect::to(&format!("/game-zone?user={}", &first_name))
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

#[derive(Deserialize, Serialize)]
pub struct GameZoneQuery {
    user: String,
}

pub async fn game_zone(
    Query(params): Query<GameZoneQuery>,
    Extension(state): Extension<Arc<State>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
    let token = auth.token();
    println!("Token: {}", token);

    if !is_valid_token(token, &state.jwt_secret) {
        Redirect::to("/404").into_response()
    } else {
        let name = params.user;
        let template = GameZoneTemplate { first_name: &name };

        Html(template.render().unwrap()).into_response()
    }
}
