use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde_json::{json, Value};

use crate::errors::{Error, Result};
use crate::web::db::Db;
use crate::web::user::{User, UpdateUserPayload};

pub fn routes(db: Db) -> Router {
    Router::new()
        .route("/users", get(list_users))
        .route(
            "/users/:id",
            get(get_user)
                .put(update_user)
                .delete(delete_user),
        )
        .with_state(db)
}

async fn list_users(State(db): State<Db>) -> Result<Json<Value>> {
    let users = sqlx::query_as::<_, User>("SELECT id, username FROM users")
        .fetch_all(&db)
        .await?;

    Ok(Json(json!({
        "status": "success",
        "data": {
            "users": users
        }
    })))
}

async fn get_user(State(db): State<Db>, Path(user_id): Path<i64>) -> Result<Json<Value>> {
    let user = sqlx::query_as::<_, User>("SELECT id, username FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&db)
        .await?
        .ok_or(Error::UserNotFound)?;

    Ok(Json(json!({
        "status": "success",
        "data": {
            "user": user
        }
    })))
}

async fn update_user(
    State(db): State<Db>,
    Path(user_id): Path<i64>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<Json<Value>> {
    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET username = $1 WHERE id = $2 RETURNING id, username",
    )
    .bind(payload.username)
    .bind(user_id)
    .fetch_optional(&db)
    .await?
    .ok_or(Error::UserNotFound)?;

    Ok(Json(json!({
        "status": "success",
        "data": {
            "user": user
        }
    })))
}

async fn delete_user(State(db): State<Db>, Path(user_id): Path<i64>) -> Result<Json<Value>> {
    let rows_affected = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(&db)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        return Err(Error::UserNotFound);
    }

    Ok(Json(json!({
        "status": "success",
        "message": "User deleted successfully"
    })))
}
