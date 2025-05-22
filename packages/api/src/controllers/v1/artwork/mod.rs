#![allow(unused)]
use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::{get, post},
    },
};
use common::Result;
use common::tables::prelude::*;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct ArtworkPathParam {
    id: i64,
}

#[derive(Clone, Debug)]
pub struct ArtworkController {
    pool: sqlx::PgPool,
    repo: ArtworkRepository,
}

impl ArtworkController {
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

impl ArtworkController {
    pub async fn list(
        State(_ctrl): State<ArtworkController>,
        Extension(_claim): Extension<Option<Authorization>>,
        Query(q): Query<ArtworkParam>,
    ) -> Result<Json<Vec<ArtworkSummary>>> {
        //TODO: Add Listing Artworks
        tracing::debug!("list artworks");
        Ok(Json(vec![]))
    }
    pub async fn get(
        State(_ctrl): State<ArtworkController>,
        Extension(_claim): Extension<Option<Authorization>>,
        Path(ArtworkPathParam { id }): Path<ArtworkPathParam>,
    ) -> Result<Json<Artwork>> {
        tracing::debug!("get artwork {id}");
        Ok(Json(Artwork::default()))
    }
    pub async fn act(
        State(_ctrl): State<ArtworkController>,
        Extension(_claim): Extension<Option<Authorization>>,
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
        State(_ctrl): State<ArtworkController>,
        Path(ArtworkPathParam { id }): Path<ArtworkPathParam>,
        Extension(_claim): Extension<Option<Authorization>>,
        Json(body): Json<ArtworkByIdAction>,
    ) -> Result<Json<Artwork>> {
        tracing::debug!("artwork act_by_id {id} {body:?}");
        match body {
            ArtworkByIdAction::Update(_) => {
                //TODO: Add Update Artwork
                Ok(Json(Artwork::default()))
            } // ArtworkByIdAction::Delete(_) => {
              //     //TODO: Add Delete Artwork
              //     Ok(Json(Artwork::default()))
              // }
        }
    }
}
