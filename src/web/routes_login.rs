use crate::{Error, Result};
use axum::{Json, Router};
use axum::routing::post;
use serde_json::{json, Value};


//Region -----Router-------
pub fn route() -> Router{
    Router::new().route("/api/login", post(api_login))
}
async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>>{
    println!("->> {:<12} - api_login {payload:?}", "HANDLER");
    // Simulate a login check
    if payload.username == "admin" && payload.password == "password" {
        Ok(Json(json!({"status": "success", "message": "Login successful"})))
    } else {
        Err(Error::LoginFail)
    }

    //TODO: ------Create db auth/db-------
}
//Region ----Define Payload-----
#[derive(Debug, serde::Deserialize)]
struct LoginPayload {
    pub username: String,
    pub password: String,
}