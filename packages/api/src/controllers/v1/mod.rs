use sqlx::{Pool, Postgres};
mod agit;
mod artist;
mod artwork;
mod collection;

use agit::AgitControllerV1;
use artist::ArtistControllerV1;
use artwork::ArtworkControllerV1;

pub fn routes(pool: Pool<Postgres>) -> models::Result<by_axum::router::BiyardRouter> {
    Ok((by_axum::router::BiyardRouter::new())
        .nest("/agits", AgitControllerV1::route(pool.clone())?)
        .nest("/artworks", ArtworkControllerV1::route(pool.clone())?)
        .nest("/artists", ArtistControllerV1::route(pool.clone())?))
}
