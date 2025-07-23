use crate::web::db::Db;
use crate::web::user::User;
use crate::{Error, Result};
use axum::extract::State;
use axum::{Json, Router};
use axum::routing::post;
use serde_json::{json, Value};
use argon2::{Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;

//Region -----Router-------
pub fn routes(db: Db) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .with_state(db)
}

//Region --- Login
async fn login(
    State(db): State<Db>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<Value>> {
    // --- Find user
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(payload.username)
        .fetch_optional(&db)
        .await?
        .ok_or(Error::LoginFail)?;

    // --- Verify password
    let password_verified = verify_password(&payload.password, &user.password.unwrap_or_default())?;
    if !password_verified {
        return Err(Error::LoginFail);
    }

    // --- Login success
    Ok(Json(json!({"status": "success", "message": "Login successful"})))
}

//Region --- Register
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
struct ReturnedUser {
    id: i64,
    username: String,
}

async fn register(
    State(db): State<Db>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<Value>> {
    let password_hash = hash_password(&payload.password)?;

    let user = sqlx::query_as::<_, ReturnedUser>(
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

//Region --- Password Hashing/Verification
fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| Error::AuthFail)?
        .to_string();
    Ok(hash)
}

fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash).map_err(|_| Error::AuthFail)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}


//Region ----Define Payload-----
#[derive(Debug, serde::Deserialize)]
struct AuthPayload {
    pub username: String,
    pub password: String,
}
