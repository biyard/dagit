#![allow(unused)]

use by_axum::aide;
use by_axum::auth::Authorization;
use by_axum::axum::extract::{Path, Query, State};
use by_axum::axum::routing::{get, post};
use by_axum::axum::{Extension, Json};
use by_types::QueryResponse;
use models::v1::agit;
use models::v1::prelude::CollectionGetResponse;
use models::{
    Result,
    error::ServiceError as ApiError,
    v1::collection::{
        Collection, CollectionAction, CollectionByIdAction, CollectionCreateRequest,
        CollectionParam, CollectionQuery, CollectionRepository, CollectionSummary,
        CollectionUpdateRequest,
    },
};
use sqlx::postgres::PgRow;

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
pub struct CollectionControllerV1 {
    pool: sqlx::PgPool,
    repo: CollectionRepository,
}

impl CollectionControllerV1 {
    async fn query(
        &self,
        auth: Option<Authorization>,
        param: CollectionQuery,
    ) -> Result<QueryResponse<CollectionSummary>> {
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        let mut total_count = 0;
        let items: Vec<CollectionSummary> = CollectionSummary::query_builder()
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

        Ok(QueryResponse { total_count, items })
    }

    async fn create(
        &self,
        auth: Option<Authorization>,
        agit_id: i64,
        CollectionCreateRequest { title }: CollectionCreateRequest,
    ) -> Result<Json<Collection>> {
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        let collection = self.repo.insert(agit_id, title).await?;
        Ok(Json(collection))
    }

    async fn update(
        &self,
        auth: Option<Authorization>,
        id: i64,
        param: CollectionUpdateRequest,
    ) -> Result<Collection> {
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        Collection::query_builder()
            .id_equals(id)
            .query()
            .fetch_one(&self.pool)
            .await
            .map_err(|_| ApiError::NotFound)?;

        let collection = self.repo.update(id, param.into()).await?;
        Ok(collection)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<Collection> {
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        Collection::query_builder()
            .id_equals(id)
            .query()
            .fetch_one(&self.pool)
            .await
            .map_err(|_| ApiError::NotFound)?;

        self.repo.delete(id).await.map_err(|_| ApiError::NotFound)
    }
}

impl CollectionControllerV1 {
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
            .route("/", post(Self::act_collection).get(Self::get_collection))
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
        Path(CollectionIdPath{ id}): Path<CollectionIdPath>,
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
        Path(CollectionIdPath{ id }): Path<CollectionIdPath>,
    ) -> Result<Json<Collection>> {
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        Ok(Json(
            Collection::query_builder()
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
