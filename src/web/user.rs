use crate::web::db::Db;
use axum::{extract::State, response::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Default)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserForList {
    id: i64,
    username: String,
}

pub async fn get_users(State(db): State<Db>) -> Json<Vec<UserForList>> {
    let users = sqlx::query_as("SELECT id, username FROM users")
        .fetch_all(&db)
        .await
        .unwrap();
    Json(users)
}
