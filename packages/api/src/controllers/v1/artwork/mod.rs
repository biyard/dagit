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
pub struct ArtworkPathParam {
    id: i64,
}

#[derive(Clone, Debug)]
pub struct ArtworkControllerV1 {
    pool: sqlx::PgPool,
    repo: ArtworkRepository,
}

impl ArtworkControllerV1 {
    pub fn new(pool: sqlx::PgPool) -> Self {
        let repo = Artwork::get_repository(pool.clone());
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

impl ArtworkControllerV1 {
    pub async fn list(
        State(ctrl): State<ArtworkControllerV1>,
        Extension(claim): Extension<Option<Authorization>>,
        Query(q): Query<ArtworkParam>,
    ) -> Result<Json<Vec<ArtworkSummary>>> {
        //TODO: Add Listing Artworks
        tracing::debug!("list artworks");
        Ok(Json(vec![]))
    }
    pub async fn get(
        State(ctrl): State<ArtworkControllerV1>,
        Extension(claim): Extension<Option<Authorization>>,
        Path(ArtworkPathParam { id }): Path<ArtworkPathParam>,
    ) -> Result<Json<Artwork>> {
        tracing::debug!("get artwork {id}");
        Ok(Json(Artwork::default()))
    }
    pub async fn act(
        State(ctrl): State<ArtworkControllerV1>,
        Extension(claim): Extension<Option<Authorization>>,
        Json(body): Json<ArtworkAction>,
    ) -> Result<Json<Artwork>> {
        tracing::debug!("artwork act {body:?}");
        match body {
            ArtworkAction::Create(req) => {
                //TODO: Add Create Artwork
                Ok(Json(Artwork::default()))
            }
        }
    }

    pub async fn act_by_id(
        State(ctrl): State<ArtworkControllerV1>,
        Path(ArtworkPathParam { id }): Path<ArtworkPathParam>,
        Extension(claim): Extension<Option<Authorization>>,
        Json(body): Json<ArtworkByIdAction>,
    ) -> Result<Json<Artwork>> {
        tracing::debug!("artwork act_by_id {id} {body:?}");
        match body {
            ArtworkByIdAction::Update(_) => {
                //TODO: Add Update Artwork
                Ok(Json(Artwork::default()))
            }
            ArtworkByIdAction::Delete(_) => {
                //TODO: Add Delete Artwork
                Ok(Json(Artwork::default()))
            }
        }
    }
}
