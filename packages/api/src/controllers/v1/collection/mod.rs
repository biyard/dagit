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
    tables::collections::{
        Collection, CollectionAction, CollectionByIdAction, CollectionCreateRequest,
        CollectionGetResponse, CollectionParam, CollectionQuery, CollectionRepository,
        CollectionSummary, CollectionUpdateRequest,
    },
};
use sqlx::postgres::PgRow;
#[cfg(test)]
mod tests;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct CollectionPathParam {
    agit_id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct CollectionIdPath {
    id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct CollectionByIdPathParam {
    agit_id: i64,
    id: i64,
}

#[derive(Clone, Debug)]
pub struct CollectionController {
    pool: sqlx::PgPool,
    repo: CollectionRepository,
}

impl CollectionController {
    async fn query(
        &self,
        auth: Option<Authorization>,
        param: CollectionQuery,
    ) -> Result<by_types::QueryResponse<CollectionSummary>> {
        // FIXME: REPLACE TO AppClaims
        let user_id = match auth {
            Some(Authorization::Bearer { claims }) => {
                let res = claims.custom.get("id");
                let mut id = 0;
                if let Some(res) = res {
                    id = res.parse().unwrap_or_default();
                }
                id
            }
            _ => 0,
        };

        let mut total_count = 0;
        let items: Vec<CollectionSummary> = CollectionSummary::query_builder(user_id)
            .limit(param.size())
            .page(param.page())
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;
                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(by_types::QueryResponse { total_count, items })
    }

    async fn create(
        &self,
        auth: Option<Authorization>,
        agit_id: i64,
        CollectionCreateRequest {
            title,
            description,
            external_link,
            banner_url,
            logo_url,
            verified,
            floor_price_eth,
            floor_change_eth,
            volume_eth,
            volume_change_24h,
            volume_change_7d,
            owners,
            status,
            stock,
        }: CollectionCreateRequest,
    ) -> Result<Json<Collection>> {
        if auth.is_none() {
            return Err(ServiceError::Unauthorized);
        }

        let collection = self
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
                floor_price_eth,
                floor_change_eth,
                volume_eth,
                volume_change_24h,
                volume_change_7d,
                owners,
                status,
                stock,
            )
            .await?;
        tracing::debug!("create collection {:?}", collection);
        Ok(Json(collection))
    }

    async fn update(
        &self,
        auth: Option<Authorization>,
        id: i64,
        param: CollectionUpdateRequest,
    ) -> Result<Collection> {
        if auth.is_none() {
            return Err(ServiceError::Unauthorized);
        }

        let collection = self.repo.update(id, param.into()).await?;
        Ok(collection)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<Collection> {
        if auth.is_none() {
            return Err(ServiceError::Unauthorized);
        }

        self.repo
            .delete(id)
            .await
            .map_err(|_| ServiceError::NotFound)
    }
}

impl CollectionController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Collection::get_repository(pool.clone());
        Self { repo, pool }
    }

    pub fn route(pool: sqlx::PgPool) -> Result<by_axum::axum::Router> {
        let ctrl = Self::new(pool);
        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_collection_by_id).post(Self::act_collection_by_id),
            )
            .route("/", get(Self::get_collection))
            .route("/create/:agit_id", post(Self::act_collection))
            .with_state(ctrl))
    }

    pub async fn act_collection(
        State(ctrl): State<Self>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(CollectionPathParam { agit_id }): Path<CollectionPathParam>,
        Json(body): Json<CollectionAction>,
    ) -> Result<Json<Collection>> {
        match body {
            CollectionAction::Create(param) => {
                let new_collection = ctrl.create(auth, agit_id, param).await?;
                Ok(new_collection)
            }
        }
    }

    pub async fn act_collection_by_id(
        State(ctrl): State<Self>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(CollectionIdPath { id }): Path<CollectionIdPath>,
        Json(body): Json<CollectionByIdAction>,
    ) -> Result<Json<Collection>> {
        tracing::debug!("act_collection_by_id {:?}", body);

        match body {
            CollectionByIdAction::Update(param) => {
                let res = ctrl.update(auth, id, param).await?;
                Ok(Json(res))
            }

            CollectionByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_collection_by_id(
        State(ctrl): State<Self>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(CollectionIdPath { id }): Path<CollectionIdPath>,
    ) -> Result<Json<Collection>> {
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
            Collection::query_builder(user_id)
                .id_equals(id)
                .query()
                .map(Collection::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_collection(
        State(ctrl): State<Self>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<CollectionParam>,
    ) -> Result<Json<CollectionGetResponse>> {
        tracing::debug!("get_collection {:?}", q);

        match q {
            CollectionParam::Query(param) => Ok(Json(CollectionGetResponse::Query(
                ctrl.query(auth, param).await?,
            ))),
        }
    }
}
