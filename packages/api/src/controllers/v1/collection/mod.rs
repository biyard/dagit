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
pub struct CollectionPathParam {
    agit_id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct CollectionByIdPathParam {
    agit_id: i64,
    id: i64,
}

#[derive(Clone, Debug)]
pub struct CollectionControllerV1 {
    pool: sqlx::PgPool,
    repo: CollectionRepository,
}

impl CollectionControllerV1 {
    pub fn new(pool: sqlx::PgPool) -> Self {
        let repo = Collection::get_repository(pool.clone());
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

impl CollectionControllerV1 {
    pub async fn list(
        State(ctrl): State<CollectionControllerV1>,
        Extension(claim): Extension<Option<Authorization>>,
        Path(CollectionPathParam { agit_id }): Path<CollectionPathParam>,
        Query(q): Query<CollectionParam>,
    ) -> Result<Json<Vec<CollectionSummary>>> {
        //TODO: Add Listing Collections
        tracing::debug!("list collections {agit_id}");
        Ok(Json(vec![]))
    }
    pub async fn get(
        State(ctrl): State<CollectionControllerV1>,
        Extension(claim): Extension<Option<Authorization>>,
        Path(CollectionByIdPathParam { agit_id, id }): Path<CollectionByIdPathParam>,
    ) -> Result<Json<Collection>> {
        tracing::debug!("get collection {agit_id} {id}");
        Ok(Json(Collection::default()))
    }
    pub async fn act(
        State(ctrl): State<CollectionControllerV1>,
        Extension(claim): Extension<Option<Authorization>>,
        Path(CollectionPathParam { agit_id }): Path<CollectionPathParam>,
        Json(body): Json<CollectionAction>,
    ) -> Result<Json<Collection>> {
        tracing::debug!("collection act {agit_id} {body:?}");
        match body {
            CollectionAction::Create(req) => {
                //TODO: Add Create Collection
                Ok(Json(Collection::default()))
            }
        }
    }

    pub async fn act_by_id(
        State(ctrl): State<CollectionControllerV1>,
        Extension(claim): Extension<Option<Authorization>>,
        Path(CollectionByIdPathParam { agit_id, id }): Path<CollectionByIdPathParam>,
        Json(body): Json<CollectionByIdAction>,
    ) -> Result<Json<Collection>> {
        tracing::debug!("collection act_by_id  {agit_id} {id} {body:?}");
        match body {
            CollectionByIdAction::Update(_) => {
                //TODO: Add Update Collection
                Ok(Json(Collection::default()))
            }
            CollectionByIdAction::Delete(_) => {
                //TODO: Add Delete Collection
                Ok(Json(Collection::default()))
            }
        }
    }
}
