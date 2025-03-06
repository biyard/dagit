#![allow(unused)]

use by_axum::aide;
use by_axum::axum::http::StatusCode;
use by_axum::auth::Authorization;
use by_axum::axum::extract::{Path, Query, State};
use by_axum::axum::routing::{get, post};
use by_axum::axum::{Extension, Json, Router};
use models::error::ServiceError;
use models::Result;
use models::v1::prelude::*;
use sqlx::query;
use std::sync::Arc;

use super::collection::CollectionControllerV1;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo)]
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

    pub fn route(pool: sqlx::PgPool) -> Result<Router> {
        let ctrl = Self::new(pool.clone());
        Ok(Router::new()
            .route("/:id", get(Self::get).post(Self::act_by_id))
            .route("/", get(Self::list).post(Self::act))
            .with_state(ctrl)
            .nest(
                "/:agit_id/collections",
                CollectionControllerV1::route(pool.clone())?,
            ))
    }

    pub async fn list(
        State(ctrl): State<AgitControllerV1>,
        Extension(claim): Extension<Option<Authorization>>,
        Query(q): Query<AgitParam>,
    ) -> Result<Json<Vec<AgitSummary>>> {
        tracing::debug!("list agits");
        let query = AgitQuery {
            size: 10,
            bookmark: None,
        };

        match ctrl.repo.find(&query).await {
            Ok(agits) => Ok(Json(agits.items)), 
            Err(e) => Err(ServiceError::Unknown(format!("Failed to retrieve agits: {}", e))),
        }
    }

    pub async fn get(
        State(ctrl): State<AgitControllerV1>,
        Extension(claim): Extension<Option<Authorization>>,
        Path(AgitPathParam { id }): Path<AgitPathParam>,
    ) -> Result<Json<Agit>> {
        tracing::debug!("get agit by id {}", id);
        let id_action = AgitReadAction { 
            // TODO (rejects the id)
            // id: id,
         };

        match ctrl.repo.find_one(&id_action).await {
            Ok(agit) => Ok(Json(agit)),
            Err(e) => Err(ServiceError::Unknown(format!("Failed to retrieve agit: {}", e).into())),
        }
    }

    pub async fn act(
        State(ctrl): State<AgitControllerV1>,
        Extension(claim): Extension<Option<Authorization>>,
        Json(body): Json<AgitAction>,
    ) -> Result<Json<Agit>> {
        tracing::debug!("agit act {body:?}");
        
        match body {
            AgitAction::Create(req) => {
                match ctrl.repo.insert(req.title).await {
                    Ok(agit) => Ok(Json(agit)),
                    Err(e) => Err(ServiceError::Unknown(format!("Failed to create agit: {}", e).into())),
                }
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
            AgitByIdAction::Update(req) => {
                let update_request = AgitRepositoryUpdateRequest {
                    title: Some(req.title),
                };

                match ctrl.repo.update(id, update_request).await {
                    Ok(updated_agit) => Ok(Json(updated_agit)),
                    Err(e) => Err(ServiceError::Unknown(format!("Failed to update agit: {}", e).into())),
                }
            }
            AgitByIdAction::Delete(_) => {
                match ctrl.repo.delete(id).await {
                    Ok(_) => Ok(Json(Agit::default())), 
                    Err(e) => Err(ServiceError::Unknown(format!("Failed to delete agit: {}", e).into())),
                }
            }
        }
    }
}
