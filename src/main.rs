mod config;
mod db;
mod error;
mod routes;
mod services;

use axum::Router;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub stripe: services::stripe_connect::StripeConnect,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = config::Config::from_env().expect("Failed to load config");
    let pool = db::connect(&cfg).await.expect("Failed to connect to db");
    db::run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    let stripe = services::stripe_connect::StripeConnect::new(&cfg);
    let state = AppState { pool, stripe };

    let app = Router::new()
        .merge(routes::router())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port));
    tracing::info!("WheelBase backend listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind port");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server failed");
}
