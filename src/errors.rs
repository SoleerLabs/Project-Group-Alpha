use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
    AuthFail,
    SqlxError(sqlx::Error),
    ProjectNotFound,
    ProjectUnauthorized,
    TaskNotFound,
    TaskUnauthorized,
    UserNotFound,
    AnyhowError(anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let (status, error_message) = match self {
            Error::LoginFail => (StatusCode::UNAUTHORIZED, "Login failed"),
            Error::AuthFail => (StatusCode::UNAUTHORIZED, "Authentication failed"),
            Error::SqlxError(err) => {
                eprintln!("->> SQLX Error: {err:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            Error::ProjectNotFound => (StatusCode::NOT_FOUND, "Project not found"),
            Error::ProjectUnauthorized => (StatusCode::FORBIDDEN, "Forbidden access to project"),
            Error::TaskNotFound => (StatusCode::NOT_FOUND, "Task not found"),
            Error::TaskUnauthorized => (StatusCode::FORBIDDEN, "Forbidden access to task"),
            Error::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            Error::AnyhowError(err) => {
                eprintln!("->> Anyhow Error: {err:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
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

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::AnyhowError(err)
    }
}