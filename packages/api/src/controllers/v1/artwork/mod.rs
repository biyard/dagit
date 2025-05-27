#![allow(unused)]
use bdk::prelude::{by_types::QueryResponse, *};
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
use sqlx::Row;
use sqlx::postgres::PgRow;

use crate::utils::app_claims::AppClaims;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct ArtworkPath {
    agit_id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct ArtworkByIdPath {
    agit_id: i64,
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
        State(ctrl): State<ArtworkController>,
        Extension(claim): Extension<Option<Authorization>>,
        Path(ArtworkPath { agit_id }): Path<ArtworkPath>,
        Query(param): Query<ArtworkParam>,
    ) -> Result<Json<ArtworkGetResponse>> {
        let user_id = match claim {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_id(),
            _ => 0,
        };
        match param {
            ArtworkParam::Query(q) => Ok(Json(ArtworkGetResponse::Query(
                ctrl.query(user_id, agit_id, q).await?,
            ))),
        }
    }
    pub async fn get(
        State(ctrl): State<ArtworkController>,
        Extension(claim): Extension<Option<Authorization>>,
        Path(ArtworkByIdPath { agit_id, id }): Path<ArtworkByIdPath>,
    ) -> Result<Json<Artwork>> {
        Ok(Json(Artwork::default()))
    }
    pub async fn act(
        State(ctrl): State<ArtworkController>,
        Extension(claim): Extension<Option<Authorization>>,
        Path(ArtworkPath { agit_id }): Path<ArtworkPath>,
        Json(body): Json<ArtworkAction>,
    ) -> Result<Json<Artwork>> {
        match body {
            ArtworkAction::Create(req) => {
                //TODO: Add Create Artwork
                Ok(Json(Artwork::default()))
            }
        }
    }
    pub async fn act_by_id(
        State(ctrl): State<ArtworkController>,
        Path(ArtworkByIdPath { agit_id, id }): Path<ArtworkByIdPath>,
        Extension(claim): Extension<Option<Authorization>>,
        Json(body): Json<ArtworkByIdAction>,
    ) -> Result<Json<Artwork>> {
        tracing::debug!("artwork act_by_id {id} {body:?}");
        match body {
            ArtworkByIdAction::Update(_) => {
                //TODO: Add Update Artwork
                Ok(Json(Artwork::default()))
            }
        }
    }
}

impl ArtworkController {
    async fn query(
        &self,
        user_id: i64,
        agit_id: i64,
        param: ArtworkQuery,
    ) -> Result<QueryResponse<ArtworkSummary>> {
        let total_count = sqlx::query("SELECT COUNT(*) FROM artworks WHERE agit_id = $1")
            .bind(agit_id)
            .map(|row: PgRow| row.get::<i64, _>(0))
            .fetch_one(&self.pool)
            .await?;

        let items: Vec<ArtworkSummary> = Artwork::query_builder(user_id)
            .limit(param.size())
            .page(param.page())
            .agit_id_equals(agit_id)
            .query()
            .map(|row: PgRow| row.into())
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }
}
