use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    AppState,
};

#[derive(Deserialize)]
pub struct StartOnboardingRequest {
    pub user_id: Uuid,
    pub email: String,
    pub country: String,
}

#[derive(Serialize)]
pub struct OnboardingResponse {
    pub account_id: String,
    pub onboarding_url: String,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/host/onboarding", post(start_onboarding))
}

async fn start_onboarding(
    State(state): State<AppState>,
    Json(req): Json<StartOnboardingRequest>,
) -> Result<Json<OnboardingResponse>> {
    let existing = sqlx::query_as::<_, (Option<String>,)>(
        "SELECT stripe_account_id FROM profiles WHERE id = $1",
    )
    .bind(req.user_id)
    .fetch_optional(&state.pool)
    .await?;

    let account_id = if let Some(row) = existing {
        match row.0 {
            Some(account_id) => account_id,
            None => {
                let account = state
                    .stripe
                    .create_host_account(&req.email, &req.country, &req.user_id.to_string())
                    .await?;
                sqlx::query("UPDATE profiles SET stripe_account_id = $1 WHERE id = $2")
                    .bind(account.id.as_str())
                    .bind(req.user_id)
                    .execute(&state.pool)
                    .await?;
                account.id.to_string()
            }
        }
    } else {
        let account = state
            .stripe
            .create_host_account(&req.email, &req.country, &req.user_id.to_string())
            .await?;
        let result = sqlx::query("UPDATE profiles SET stripe_account_id = $1 WHERE id = $2")
            .bind(account.id.as_str())
            .bind(req.user_id)
            .execute(&state.pool)
            .await?;
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Profile not found".into()));
        }
        account.id.to_string()
    };

    Ok(Json(OnboardingResponse {
        account_id: account_id.clone(),
        onboarding_url: state.stripe.onboarding_url(&account_id).await?,
    }))
}
