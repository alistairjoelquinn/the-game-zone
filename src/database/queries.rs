use crate::model::User;
use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;

pub async fn fetch_user_by_first_name(
    pool: &PgPool,
    first_name: &String,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE first_name = $1")
        .bind(first_name)
        .fetch_one(pool)
        .await
}

pub async fn fetch_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await
}

// kept as an example for upcoming update logic
pub async fn _update_user(
    pool: &PgPool,
    user: User,
    id: i32,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "UPDATE users SET first_name = $1, last_name = $2, email = $3, password_hash = $4 WHERE id = $5 RETURNING *",
    )
    .bind(user.first_name)
    .bind(user.last_name)
    .bind(user.username)
    .bind(hash(&user.password_hash, DEFAULT_COST).unwrap())
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn fetch_all_games(pool: &PgPool) -> Result<Vec<Game>, sqlx::Error> {
    sqlx::query_as::<_, Game>("SELECT * FROM games")
        .fetch_all(pool)
        .await
}
