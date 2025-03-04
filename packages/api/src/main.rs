pub mod config;
pub mod controllers;

use controllers::v1;

use by_types::DatabaseConfig;
use models::{agit::Agit, artist::Artist, artwork::Artwork, collection::Collection};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

async fn migration(pool: &sqlx::Pool<sqlx::Postgres>) -> models::Result<()> {
    //TODO: Add Model Migration
    let agit = Agit::get_repository(pool.clone());
    let collection = Collection::get_repository(pool.clone());
    let artwork = Artwork::get_repository(pool.clone());

    let artist = Artist::get_repository(pool.clone());

    agit.create_this_table().await?;
    collection.create_this_table().await?;
    artist.create_this_table().await?;
    artwork.create_this_table().await?;

    agit.create_related_tables().await?;
    collection.create_related_tables().await?;
    artist.create_related_tables().await?;
    artwork.create_related_tables().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> models::Result<()> {
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

    let app = app.nest("/v1", v1::routes(pool.clone())?);
    let port = option_env!("PORT").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}
