use bdk::prelude::*;

use sqlx::{Pool, Postgres};
mod agit;
mod artist;
mod artwork;
mod collection;
mod collector;
mod users;

use agit::AgitController;
use artist::ArtistController;
use collection::CollectionController;
use collector::CollectorController;
use users::UserController;

pub fn routes(pool: Pool<Postgres>) -> common::Result<by_axum::router::BiyardRouter> {
    Ok((by_axum::router::BiyardRouter::new())
        .nest("/agits", AgitController::route(pool.clone())?)
        .nest("/artists", ArtistController::route(pool.clone())?)
        .nest("/users", UserController::new(pool.clone()).route()?)
        .nest("/collectors", CollectorController::route(pool.clone())?)
        .nest("/collections", CollectionController::route(pool.clone())?))
}
