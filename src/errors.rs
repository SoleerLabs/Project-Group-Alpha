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
    // If you have `NoAuthCtx` from previous snippets, you might want to add it here
    // NoAuthCtx, // Example if you have this variant
    AnyhowError(anyhow::Error), // Also add this if you use `anyhow::Error` for general errors
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let (status, error_message) = match self {
            Error::LoginFail => (StatusCode::UNAUTHORIZED, "Login failed"),
            // Changed AuthFail to UNAUTHORIZED, as it's typically an client-side authentication issue
            Error::AuthFail => (StatusCode::UNAUTHORIZED, "Authentication failed"),
            // If you have NoAuthCtx, handle it similarly
            // Error::NoAuthCtx => (StatusCode::UNAUTHORIZED, "No authentication context"),

            Error::SqlxError(err) => {
                eprintln!("->> SQLX Error: {err:?}"); // Print full SQLx error for debugging
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            // ADDED: Handling for ProjectNotFound
            Error::ProjectNotFound => (StatusCode::NOT_FOUND, "Project not found"),
            // ADDED: Handling for ProjectUnauthorized
            Error::ProjectUnauthorized => (StatusCode::FORBIDDEN, "Forbidden access to project"),
            // ADDED: Handling for AnyhowError (if you introduced it)
            Error::AnyhowError(err) => {
                eprintln!("->> Anyhow Error: {err:?}"); // Print full anyhow error for debugging
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

// Add From<anyhow::Error> if you have Error::AnyhowError
impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::AnyhowError(err)
    }
}