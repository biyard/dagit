pub mod config;

use by_types::DatabaseConfig;
use sqlx::postgres::PgPoolOptions;
use thiserror::Error;
use tokio::net::TcpListener;

async fn migration(_pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), Error> {
    //TODO: Add Model Migration
    Ok(())
}

#[derive(Error, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Error {
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = by_axum::new();
    let conf = config::get();
    tracing::debug!("config: {:?}", conf);

    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await
            .expect("Failed to connect to Postgres")
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    migration(&pool).await?;

    let port = option_env!("PORT").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}
