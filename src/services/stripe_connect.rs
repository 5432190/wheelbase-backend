use std::str::FromStr;
use stripe::{
    Account, AccountLink, AccountLinkType, CheckoutSession, CreateAccount,
    CreateAccountCapabilities, CreateAccountCapabilitiesCardPayments,
    CreateAccountCapabilitiesTransfers, CreateAccountLink, CreateCheckoutSession,
    CreateCheckoutSessionLineItems, Currency, Transfer, Webhook,
};

use crate::{
    config::Config,
    error::{AppError, Result},
};

#[derive(Clone)]
pub struct StripeConnect {
    client: stripe::Client,
    platform_account: String,
    app_url: String,
    webhook_secret: String,
}

impl StripeConnect {
    pub fn new(cfg: &Config) -> Self {
        Self {
            client: stripe::Client::new(&cfg.stripe_secret_key),
            platform_account: cfg.stripe_platform_account.clone(),
            app_url: cfg.app_url.clone(),
            webhook_secret: cfg.stripe_webhook_secret.clone(),
        }
    }

    pub async fn create_host_account(
        &self,
        email: &str,
        country: &str,
        user_id: &str,
    ) -> Result<Account> {
        let mut params = CreateAccount::new();
        params.type_ = Some(stripe::AccountType::Express);
        params.email = Some(email);
        params.country = Some(country);
        params.capabilities = Some(CreateAccountCapabilities {
            card_payments: Some(CreateAccountCapabilitiesCardPayments {
                requested: Some(true),
            }),
            transfers: Some(CreateAccountCapabilitiesTransfers {
                requested: Some(true),
            }),
            ..Default::default()
        });
        params.metadata = Some(
            [("user_id".to_string(), user_id.to_string())]
                .iter()
                .cloned()
                .collect(),
        );
        Ok(Account::create(&self.client, params).await?)
    }

    pub async fn onboarding_url(&self, account_id: &str) -> Result<String> {
        let refresh_url = format!("{}/host/onboarding/refresh", self.app_url);
        let return_url = format!("{}/host/onboarding/complete", self.app_url);
        let account = stripe::AccountId::from_str(account_id)
            .map_err(|_| AppError::Validation("Invalid Stripe account id".into()))?;
        let mut params = CreateAccountLink::new(account, AccountLinkType::AccountOnboarding);
        params.refresh_url = Some(&refresh_url);
        params.return_url = Some(&return_url);
        let link = AccountLink::create(&self.client, params).await?;
        Ok(link.url)
    }

    pub async fn create_checkout_session(
        &self,
        booking_id: &str,
        amount_cents: i64,
        description: &str,
    ) -> Result<CheckoutSession> {
        let success_url = format!("{}/booking/success?booking_id={}", self.app_url, booking_id);
        let cancel_url = format!("{}/booking/cancel", self.app_url);
        let mut params = CreateCheckoutSession::new();
        params.mode = Some(stripe::CheckoutSessionMode::Payment);
        params.success_url = Some(&success_url);
        params.cancel_url = Some(&cancel_url);
        params.line_items = Some(vec![CreateCheckoutSessionLineItems {
            price_data: Some(stripe::CreateCheckoutSessionLineItemsPriceData {
                currency: Currency::USD,
                unit_amount: Some(amount_cents),
                product_data: Some(stripe::CreateCheckoutSessionLineItemsPriceDataProductData {
                    name: description.to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            quantity: Some(1),
            ..Default::default()
        }]);
        params.metadata = Some(
            [("booking_id".to_string(), booking_id.to_string())]
                .iter()
                .cloned()
                .collect(),
        );
        Ok(CheckoutSession::create(&self.client, params).await?)
    }

    pub async fn payout_host(
        &self,
        booking_id: &str,
        host_stripe_id: &str,
        gross_cents: i64,
        fee_percent: i64,
    ) -> Result<Transfer> {
        let fee = (gross_cents * fee_percent) / 100;
        let amount = gross_cents - fee;
        let mut params = stripe::CreateTransfer::new(Currency::USD, host_stripe_id.to_string());
        params.amount = Some(amount);
        params.transfer_group = Some(booking_id);
        params.metadata = Some(
            [
                ("booking_id".to_string(), booking_id.to_string()),
                ("platform_fee_cents".to_string(), fee.to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
        );
        Ok(Transfer::create(&self.client, params).await?)
    }

    pub fn verify_webhook(&self, payload: &[u8], sig: &str) -> Result<stripe::Event> {
        let payload = std::str::from_utf8(payload)
            .map_err(|_| AppError::Validation("Webhook payload must be UTF-8".into()))?;
        Ok(Webhook::construct_event(
            payload,
            sig,
            &self.webhook_secret,
        )?)
    }

    pub fn platform_account(&self) -> &str {
        &self.platform_account
    }
}
