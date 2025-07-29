pub mod routes;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Default, Clone, ToSchema)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip)]
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserPayload {
    pub username: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserForList {
    id: i64,
    username: String,
}
