#![allow(unused)]

use by_axum::aide;
use by_axum::auth::Authorization;
use by_axum::axum::extract::{Path, Query, State};
use by_axum::axum::routing::post;
use by_axum::axum::{Extension, Json};
use models::Result;
use models::v1::prelude::*;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct ArtistPathParam {
    id: i64,
}

#[derive(Clone, Debug)]
pub struct ArtistControllerV1 {
    pool: sqlx::PgPool,
    repo: ArtistRepository,
}

impl ArtistControllerV1 {
    pub fn new(pool: sqlx::PgPool) -> Self {
        let repo = Artist::get_repository(pool.clone());
        Self { repo, pool }
    }

    pub fn route(pool: sqlx::PgPool) -> Result<by_axum::axum::Router> {
        let ctrl = Self::new(pool);
        Ok(by_axum::axum::Router::new()
            .route("/:id", post(Self::act_by_id).get(Self::get))
            .route("/", post(Self::act).get(Self::list))
            .with_state(ctrl))
    }
}

impl ArtistControllerV1 {
    pub async fn list(
        State(ctrl): State<ArtistControllerV1>,
        Extension(claim): Extension<Option<Authorization>>,
        Query(q): Query<ArtistParam>,
    ) -> Result<Json<Vec<ArtistSummary>>> {
        //TODO: Add Listing Artists
        tracing::debug!("list artists");
        Ok(Json(vec![]))
    }
    pub async fn get(
        State(ctrl): State<ArtistControllerV1>,
        Extension(claim): Extension<Option<Authorization>>,
        Path(ArtistPathParam { id }): Path<ArtistPathParam>,
    ) -> Result<Json<Artist>> {
        tracing::debug!("get artist {id}");
        Ok(Json(Artist::default()))
    }
    pub async fn act(
        State(ctrl): State<ArtistControllerV1>,
        Extension(claim): Extension<Option<Authorization>>,
        Json(body): Json<ArtistAction>,
    ) -> Result<Json<Artist>> {
        tracing::debug!("artist act {body:?}");
        match body {
            ArtistAction::Create(req) => {
                //TODO: Add Create Artist
                Ok(Json(Artist::default()))
            }
        }
    }

    pub async fn act_by_id(
        State(ctrl): State<ArtistControllerV1>,
        Path(ArtistPathParam { id }): Path<ArtistPathParam>,
        Extension(claim): Extension<Option<Authorization>>,
        Json(body): Json<ArtistByIdAction>,
    ) -> Result<Json<Artist>> {
        tracing::debug!("artist act_by_id {id} {body:?}");
        match body {
            ArtistByIdAction::Update(_) => {
                //TODO: Add Update Artist
                Ok(Json(Artist::default()))
            }
            ArtistByIdAction::Delete(_) => {
                //TODO: Add Delete Artist
                Ok(Json(Artist::default()))
            }
        }
    }
}
