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

use common::{
    Result,
    error::ServiceError,
    tables::collectors::{
        Collector, CollectorAction, CollectorByIdAction, CollectorCreateRequest,
        CollectorGetResponse, CollectorParam, CollectorQuery, CollectorRepository,
        CollectorSummary, CollectorUpdateRequest,
    },
};

use sqlx::postgres::PgRow;
#[cfg(test)]
mod tests;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct CollectorPathParam {
    agit_id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct CollectorIdPath {
    id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]

pub struct CollectorByIdPathParam {
    id: i64,
    agit_id: i64,
}

#[derive(Clone, Debug)]
pub struct CollectorController {
    pool: sqlx::PgPool,
    repo: CollectorRepository,
}

impl CollectorController {
    async fn query(
        &self,
        auth: Option<Authorization>,
        param: CollectorQuery,
    ) -> Result<by_types::QueryResponse<CollectorSummary>> {
        let user_id = match auth {
            Some(Authorization::Bearer { claims }) => claims
                .custom
                .get("id")
                .ok_or(ServiceError::Unauthorized)?
                .parse()
                .map_err(|e| {
                    tracing::error!("failed to parse id {e}");
                    ServiceError::Unauthorized
                })?,
            _ => {
                return Err(ServiceError::Unauthorized);
            }
        };

        let mut total_count = 0;

        let items: Vec<CollectorSummary> = CollectorSummary::query_builder(user_id)
            .limit(param.size())
            .page(param.page())
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;
                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .collect();
        Ok(by_types::QueryResponse { total_count, items })
    }

    async fn create(
        &self,
        auth: Option<Authorization>,
        agit_id: i64,

        CollectorCreateRequest {
            title,
            description,
            external_link,
            banner_url,
            logo_url,
            verified,
            owned,
            token_ids,
            wallet_address,
            total_volume,
            last_activity,
        }: CollectorCreateRequest,
    ) -> Result<Json<Collector>> {
        if auth.is_none() {
            return Err(ServiceError::Unauthorized);
        }

        let collector = self
            .repo
            .insert(
                agit_id,
                title,
                description,
                external_link,
                banner_url,
                logo_url,
                false,
                verified,
                owned,
                token_ids,
                wallet_address,
                total_volume,
                last_activity,
            )
            .await?;
        Ok(Json(collector))
    }

    async fn update(
        &self,
        auth: Option<Authorization>,
        id: i64,
        param: CollectorUpdateRequest,
    ) -> Result<Collector> {
        if auth.is_none() {
            return Err(ServiceError::Unauthorized);
        }
        let collector = self.repo.update(id, param.into()).await?;
        Ok(collector)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<Collector> {
        if auth.is_none() {
            return Err(ServiceError::Unauthorized);
        }
        self.repo
            .delete(id)
            .await
            .map_err(|_| ServiceError::NotFound)
    }
}

impl CollectorController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Collector::get_repository(pool.clone());
        Self { repo, pool }
    }

    pub fn route(pool: sqlx::PgPool) -> Result<by_axum::axum::Router> {
        let ctrl = Self::new(pool);
        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_collector_by_id).post(Self::act_collector_by_id),
            )
            .route("/", get(Self::get_collector))
            .route("/create/:agit_id", post(Self::act_collector))
            .with_state(ctrl))
    }

    pub async fn act_collector(
        State(ctrl): State<Self>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(CollectorPathParam { agit_id }): Path<CollectorPathParam>,
        Json(body): Json<CollectorAction>,
    ) -> Result<Json<Collector>> {
        match body {
            CollectorAction::Create(param) => {
                let new_collector = ctrl.create(auth, agit_id, param).await?;
                Ok(new_collector)
            }
        }
    }

    pub async fn act_collector_by_id(
        State(ctrl): State<Self>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(CollectorIdPath { id }): Path<CollectorIdPath>,
        Json(body): Json<CollectorByIdAction>,
    ) -> Result<Json<Collector>> {
        tracing::debug!("act_collector_by_id {:?}", body);
        match body {
            CollectorByIdAction::Update(param) => {
                let collector = ctrl.update(auth, id, param).await?;
                Ok(Json(collector))
            }
            CollectorByIdAction::Delete(_) => {
                let collector = ctrl.delete(id, auth).await?;
                Ok(Json(collector))
            }
        }
    }

    pub async fn get_collector_by_id(
        State(ctrl): State<Self>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(CollectorIdPath { id }): Path<CollectorIdPath>,
    ) -> Result<Json<Collector>> {
        let user_id = match auth {
            Some(Authorization::Bearer { claims }) => claims
                .custom
                .get("id")
                .ok_or(ServiceError::Unauthorized)?
                .parse()
                .map_err(|e| {
                    tracing::error!("failed to parse id {e}");
                    ServiceError::Unauthorized
                })?,
            _ => {
                return Err(ServiceError::Unauthorized);
            }
        };

        Ok(Json(
            Collector::query_builder(user_id)
                .id_equals(id)
                .query()
                .map(Collector::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }
    pub async fn get_collector(
        State(ctrl): State<Self>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(param): Query<CollectorParam>,
    ) -> Result<Json<CollectorGetResponse>> {
        tracing::debug!("get_collector {param}");

        match param {
            CollectorParam::Query(param) => Ok(Json(CollectorGetResponse::Query(
                ctrl.query(auth, param).await?,
            ))),
        }
    }
}
