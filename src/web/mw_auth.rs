use crate::ctx::Ctx;
use crate::web::db::Db;
use crate::web::user::User;
use crate::{Error, Result};
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future::Future;

impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    fn from_request_parts<'a, 'b>(
        parts: &'a mut Parts,
        _state: &'b S,
    ) -> impl Future<Output = std::result::Result<Self, Self::Rejection>> + Send {
        async move {
            println!("->> {:<12} - Ctx::from_request_parts", "EXTRACTOR");

            parts
                .extensions
                .get::<Ctx>()
                .cloned()
                .ok_or(Error::AuthFail)
        }
    }
}

pub async fn mw_auth(
    State(db): State<Db>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_auth", "MIDDLEWARE");

    let token = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(Error::AuthFail)?;

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::default();
    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|_| Error::AuthFail)?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(token_data.claims.sub)
        .fetch_optional(&db)
        .await?
        .ok_or(Error::AuthFail)?;

    let ctx = Ctx { user };
    req.extensions_mut().insert(ctx);

    Ok(next.run(req).await)
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i64,
    exp: usize,
}