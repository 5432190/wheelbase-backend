use crate::config::Config;
use sqlx::PgPool;

pub async fn connect(cfg: &Config) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(&cfg.database_url).await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!().run(pool).await
}
