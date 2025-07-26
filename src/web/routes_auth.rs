use crate::ctx::Ctx;
use crate::web::db::Db;
use crate::web::user::User;
use crate::{Error, Result};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub fn routes(db: Db) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/me", get(me))
        .with_state(db)
}

async fn me(ctx: Ctx) -> Result<Json<Value>> {
    Ok(Json(json!({
        "status": "success",
        "data": {
            "user": {
                "id": ctx.user.id,
                "username": ctx.user.username
            }
        }
    })))
}

async fn login(State(db): State<Db>, Json(payload): Json<AuthPayload>) -> Result<Json<Value>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(payload.username)
        .fetch_optional(&db)
        .await?
        .ok_or(Error::LoginFail)?;

    let password_verified = verify_password(&payload.password, &user.password.unwrap_or_default())?;
    if !password_verified {
        return Err(Error::LoginFail);
    }

    let token = create_token(user.id)?;

    Ok(Json(json!({
        "status": "success",
        "data": {
            "token": token
        }
    })))
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
struct ReturnedUser {
    id: i64,
    username: String,
}

async fn register(State(db): State<Db>, Json(payload): Json<AuthPayload>) -> Result<Json<Value>> {
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

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i64,
    exp: usize,
}

fn create_token(user_id: i64) -> Result<String> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| Error::AuthFail)
}

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

#[derive(Debug, serde::Deserialize)]
struct AuthPayload {
    pub username: String,
    pub password: String,
}
