use crate::web::db::Db;
use crate::web::user::User;
use crate::{Error, Result};
use axum::extract::State;
use axum::{Json, Router};
use axum::routing::post;
use serde_json::{json, Value};
use argon2::{Argon2, PasswordHasher, SaltString};
use rand::rngs::OsRng;

//Region -----Router-------
pub fn routes(db: Db) -> Router {
    Router::new()
        .route("/login", post(api_login))
        .route("/register", post(api_register))
        .with_state(db)
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login {payload:?}", "HANDLER");
    // Simulate a login check
    if payload.username == "admin" && payload.password == "password" {
        Ok(Json(json!({"status": "success", "message": "Login successful"})))
    } else {
        Err(Error::LoginFail)
    }

    //TODO: ------Create db auth/db-------
}

async fn api_register(
    State(db): State<Db>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    let password_hash = hash_password(&payload.password)?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id, username",
    )
    .bind(payload.username)
    .bind(password_hash)
    .fetch_one(&db)
    .await?;

    Ok(Json(json!({
        "status": "success",
        "data": {
            "user": {
                "id": user.id,
                "username": user.username
            }
        }
    })))
}

fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| Error::AuthFail)?
        .to_string();
    Ok(hash)
}

//Region ----Define Payload-----
#[derive(Debug, serde::Deserialize)]
struct LoginPayload {
    pub username: String,
    pub password: String,
}