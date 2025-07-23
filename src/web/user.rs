use crate::web::db::Db;
use axum::{extract::State, response::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Default)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: Option<String>,
}

pub async fn get_users(State(db): State<Db>) -> Json<Vec<User>> {
    let users = sqlx::query_as::<_, User>("SELECT id, username FROM users")
        .fetch_all(&db)
        .await
        .unwrap();
    Json(users)
}
