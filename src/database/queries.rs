use crate::model::User;
use crate::utils::auth::hash_password;
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
    .bind(hash_password(&user.password_hash).unwrap())
    .bind(id)
    .fetch_one(pool)
    .await
}
