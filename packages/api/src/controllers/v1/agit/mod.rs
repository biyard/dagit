#![allow(unused)]

use by_axum::aide;
use by_axum::auth::Authorization;
use by_axum::axum::extract::{Path, Query, State};
use by_axum::axum::routing::post;
use by_axum::axum::{Extension, Json};
use models::Result;
use models::v1::prelude::*;

use super::collection::CollectionControllerV1;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct AgitPathParam {
    id: i64,
}

#[derive(Clone, Debug)]
pub struct AgitControllerV1 {
    pool: sqlx::PgPool,
    repo: AgitRepository,
}

impl AgitControllerV1 {
    pub fn new(pool: sqlx::PgPool) -> Self {
        let repo = Agit::get_repository(pool.clone());
        Self { repo, pool }
    }

    pub fn route(pool: sqlx::PgPool) -> Result<by_axum::axum::Router> {
        let ctrl = Self::new(pool.clone());
        Ok(by_axum::axum::Router::new()
            .route("/:id", post(Self::act_by_id).get(Self::get))
            .route("/", post(Self::act).get(Self::list))
            .with_state(ctrl)
            .nest(
                "/:agit_id/collections",
                CollectionControllerV1::route(pool.clone())?,
            ))
    }
}

impl AgitControllerV1 {
    pub async fn list(
        State(ctrl): State<AgitControllerV1>,
        Extension(claim): Extension<Option<Authorization>>,
        Query(q): Query<AgitParam>,
    ) -> Result<Json<Vec<AgitSummary>>> {
        //TODO: Add Listing Agits
        tracing::debug!("list agits");
        Ok(Json(vec![]))
    }
    pub async fn get(
        State(ctrl): State<AgitControllerV1>,
        Extension(claim): Extension<Option<Authorization>>,
        Path(AgitPathParam { id }): Path<AgitPathParam>,
    ) -> Result<Json<Agit>> {
        tracing::debug!("get agit {id}");
        Ok(Json(Agit::default()))
    }
    pub async fn act(
        State(ctrl): State<AgitControllerV1>,
        Extension(claim): Extension<Option<Authorization>>,
        Json(body): Json<AgitAction>,
    ) -> Result<Json<Agit>> {
        tracing::debug!("agit act {body:?}");
        match body {
            AgitAction::Create(req) => {
                //TODO: Add Create Agit
                Ok(Json(Agit::default()))
            }
        }
    }

    pub async fn act_by_id(
        State(ctrl): State<AgitControllerV1>,
        Path(AgitPathParam { id }): Path<AgitPathParam>,
        Extension(claim): Extension<Option<Authorization>>,
        Json(body): Json<AgitByIdAction>,
    ) -> Result<Json<Agit>> {
        tracing::debug!("agit act_by_id {id} {body:?}");
        match body {
            AgitByIdAction::Update(_) => {
                //TODO: Add Update Agit
                Ok(Json(Agit::default()))
            }
            AgitByIdAction::Delete(_) => {
                //TODO: Add Delete Agit
                Ok(Json(Agit::default()))
            }
        }
    }
}
