use crate::{
    database::queries,
    model::{
        GameZoneTemplate, HomeTemplate, LoginFieldTemplate, User,
        WrongPasswordTemplate,
    },
    state::State,
};
use askama::Template;
use axum::{
    extract::{Form, Query},
    response::{Html, IntoResponse, Redirect},
    Extension, Json,
};
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
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

#[derive(Deserialize, Serialize)]
pub struct GameZoneQuery {
    user: String,
}

pub async fn game_zone(
    Query(params): Query<GameZoneQuery>,
    TypedHeader(auth): TypedHeader<Authorization>,
) -> impl IntoResponse {
    let token = auth.token();
    println!("Token: {}", token);

    if !is_valid_token(token) {
        Redirect::to(&format!("/404")).into_response()
    } else {
        let name = params.user;
        let template = GameZoneTemplate { first_name: &name };

        Html(template.render().unwrap()).into_response()
    }
}
