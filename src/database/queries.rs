use crate::model::User;
use sqlx::PgPool;

pub async fn create_user(
    pool: &PgPool,
    user: User,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (first_name, last_name, email, password) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(user.first_name)
    .bind(user.last_name)
    .bind(user.username)
    .bind(user.password)
    .fetch_one(pool)
    .await
}

pub async fn fetch_user_by_id(
    pool: &PgPool,
    id: i32,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn fetch_user_by_first_name(
    pool: &PgPool,
    first_name: &String,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(first_name)
        .fetch_one(pool)
        .await
}

pub async fn fetch_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await
}

pub async fn update_user(
    pool: &PgPool,
    user: User,
    id: i32,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "UPDATE users SET first_name = $1, last_name = $2, email = $3, password = $4 WHERE id = $5 RETURNING *",
    )
    .bind(user.first_name)
    .bind(user.last_name)
    .bind(user.username)
    .bind(user.password)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn delete_user_by_id(
    pool: &PgPool,
    id: i32,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("DELETE FROM users WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_one(pool)
        .await
}
