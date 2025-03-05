use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub fn routes(pool: Pool<Postgres>) -> models::Result<by_axum::router::BiyardRouter> {
    Ok((by_axum::router::BiyardRouter::new())
        .nest("/agits", controllers::v1::agit::routes(pool.clone())?)
        .nest("/artists", controllers::v1::artist::routes(pool.clone())?)
        .nest("/artworks", controllers::v1::artwork::routes(pool.clone())?)
        .nest(
            "/collections",
            controllers::v1::collection::routes(pool.clone())?,
        ))
}
