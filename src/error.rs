use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("Stripe error: {0}")]
    Stripe(#[from] stripe::StripeError),
    #[error("Stripe webhook error: {0}")]
    StripeWebhook(#[from] stripe::WebhookError),
    #[error("Validation failed: {0}")]
    Validation(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Internal error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Db(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database: {}", e),
            ),
            AppError::Stripe(e) => (StatusCode::BAD_GATEWAY, format!("Stripe: {}", e)),
            AppError::StripeWebhook(e) => {
                (StatusCode::BAD_REQUEST, format!("Stripe webhook: {}", e))
            }
            AppError::Validation(e) => (StatusCode::BAD_REQUEST, e.clone()),
            AppError::Conflict(e) => (StatusCode::CONFLICT, e.clone()),
            AppError::NotFound(e) => (StatusCode::NOT_FOUND, e.clone()),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".into()),
        };
        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
