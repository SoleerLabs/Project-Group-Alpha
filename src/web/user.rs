
use serde::{Deserialize, Serialize};
use axum::response::Json;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

pub async fn get_users() -> Json<Vec<User>> {
    let users = vec![
        User { id: 1, username: "Alice".to_string() },
        User { id: 2, username: "Bob".to_string() },
    ];
    Json(users)
}
