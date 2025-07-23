use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
    AuthFail,
    SqlxError(sqlx::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let (status, error_message) = match self {
            Error::LoginFail => (StatusCode::UNAUTHORIZED, "Login failed"),
            Error::AuthFail => (StatusCode::INTERNAL_SERVER_ERROR, "Authentication failed"),
            Error::SqlxError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
        };

        let body = json!({
            "status": "error",
            "message": error_message,
        });

        (status, axum::Json(body)).into_response()
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::SqlxError(err)
    }
}
