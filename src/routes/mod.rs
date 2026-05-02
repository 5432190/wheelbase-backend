use crate::AppState;
use axum::Router;

pub mod bookings;
pub mod health;
pub mod host;
pub mod webhook;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(health::router())
        .merge(bookings::router())
        .merge(host::router())
        .merge(webhook::router())
}
