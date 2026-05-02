use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub stripe_secret_key: String,
    pub stripe_webhook_secret: String,
    pub stripe_platform_account: String,
    pub app_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        dotenvy::dotenv().ok();
        Ok(Self {
            database_url: env::var("DATABASE_URL").map_err(|_| "DATABASE_URL required")?,
            stripe_secret_key: env::var("STRIPE_SECRET_KEY")
                .map_err(|_| "STRIPE_SECRET_KEY required")?,
            stripe_webhook_secret: env::var("STRIPE_WEBHOOK_SECRET")
                .map_err(|_| "STRIPE_WEBHOOK_SECRET required")?,
            stripe_platform_account: env::var("STRIPE_PLATFORM_ACCOUNT")
                .map_err(|_| "STRIPE_PLATFORM_ACCOUNT required")?,
            app_url: env::var("APP_URL").unwrap_or_else(|_| "http://localhost:3000".into()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .map_err(|_| "PORT must be a number")?,
        })
    }
}
