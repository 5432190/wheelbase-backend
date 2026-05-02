use axum::{body::Bytes, extract::State, http::HeaderMap, routing::post, Router};
use stripe::{EventObject, EventType};

use crate::{error::Result, AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/webhook/stripe", post(handle_stripe_webhook))
}

async fn handle_stripe_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<&'static str> {
    let sig = headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| crate::error::AppError::Validation("Missing stripe-signature".into()))?;

    let event = state.stripe.verify_webhook(&body, sig)?;

    match event.type_ {
        EventType::CheckoutSessionCompleted => {
            if let EventObject::CheckoutSession(session) = event.data.object {
                if let Some(booking_id_str) =
                    session.metadata.as_ref().and_then(|m| m.get("booking_id"))
                {
                    if let Ok(booking_id) = uuid::Uuid::parse_str(booking_id_str) {
                        crate::services::booking::mark_paid(&state.pool, booking_id).await?;
                    }
                }
            }
        }
        _ => {}
    }

    Ok("ok")
}
