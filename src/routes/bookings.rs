use axum::{extract::State, routing::post, Json, Router};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use crate::{error::Result, services::booking::create_booking, AppState};

#[derive(Deserialize)]
pub struct CreateBookingRequest {
    pub vehicle_id: Uuid,
    pub renter_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Serialize)]
pub struct CreateBookingResponse {
    pub booking_id: Uuid,
    pub total_cents: i64,
    pub platform_fee_cents: i64,
    pub checkout_url: String,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/bookings", post(create_booking_handler))
}

async fn create_booking_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateBookingRequest>,
) -> Result<Json<CreateBookingResponse>> {
    let vehicle =
        sqlx::query("SELECT nightly_rate, title FROM vehicles WHERE id = $1 AND status = 'active'")
            .bind(req.vehicle_id)
            .fetch_one(&state.pool)
            .await
            .map_err(|_| crate::error::AppError::NotFound("Vehicle not found".into()))?;
    let nightly_rate: i64 = vehicle.get("nightly_rate");
    let title: String = vehicle.get("title");

    let (booking_id, total_cents, platform_fee_cents) = create_booking(
        &state.pool,
        req.vehicle_id,
        req.renter_id,
        req.start_date,
        req.end_date,
        nightly_rate,
    )
    .await?;

    let session = state
        .stripe
        .create_checkout_session(
            &booking_id.to_string(),
            total_cents,
            &format!("WheelBase Booking: {}", title),
        )
        .await?;

    sqlx::query("UPDATE bookings SET stripe_session_id = $1 WHERE id = $2")
        .bind(session.id.to_string())
        .bind(booking_id)
        .execute(&state.pool)
        .await?;

    Ok(Json(CreateBookingResponse {
        booking_id,
        total_cents,
        platform_fee_cents,
        checkout_url: session
            .url
            .ok_or_else(|| crate::error::AppError::Internal)?,
    }))
}
