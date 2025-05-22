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
    tables::agits::{
        Agit, AgitAction, AgitByIdAction, AgitCreateRequest, AgitGetResponse, AgitParam, AgitQuery,
        AgitRepository, AgitSummary, AgitUpdateRequest,
    },
};
use sqlx::postgres::PgRow;

use super::artwork::ArtworkController;
#[cfg(test)]
mod tests;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct AgitPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct AgitController {
    repo: AgitRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl AgitController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Agit::get_repository(pool.clone());
        Self { repo, pool }
    }

    pub fn route(pool: sqlx::PgPool) -> Result<by_axum::axum::Router> {
        let ctrl = Self::new(pool.clone());
        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_agit).get(Self::get_agit))
            .route(
                "/:agit_id",
                get(Self::get_agit_by_id).post(Self::act_agit_by_id),
            )
            .with_state(ctrl)
            .nest(
                "/:agit_id/artworks",
                ArtworkController::route(pool.clone())?,
            ))
    }

    pub async fn act_agit(
        State(ctrl): State<AgitController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<AgitAction>,
    ) -> Result<Json<Agit>> {
        tracing::debug!("act_agit {:?}", body);

        match body {
            AgitAction::Create(param) => {
                let res = ctrl.create(auth, param).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn act_agit_by_id(
        State(ctrl): State<AgitController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(AgitPath { id }): Path<AgitPath>,
        Json(body): Json<AgitByIdAction>,
    ) -> Result<Json<Agit>> {
        tracing::debug!("act_agit_by_id {} {:?}", id, body);

        match body {
            AgitByIdAction::Update(param) => {
                let res = ctrl.update(id, auth, param).await?;
                Ok(Json(res))
            }
            AgitByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_agit_by_id(
        State(ctrl): State<AgitController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(AgitPath { id }): Path<AgitPath>,
    ) -> Result<Json<Agit>> {
        tracing::debug!("get_agit {}", id);

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
            Agit::query_builder(user_id)
                .id_equals(id)
                .query()
                .map(Agit::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_agit(
        State(ctrl): State<AgitController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<AgitParam>,
    ) -> Result<Json<AgitGetResponse>> {
        tracing::debug!("list_agit {}", q);

        match q {
            AgitParam::Query(param) => {
                Ok(Json(AgitGetResponse::Query(ctrl.query(auth, param).await?)))
            }
        }
    }
}

impl AgitController {
    async fn query(
        &self,
        auth: Option<Authorization>,
        param: AgitQuery,
    ) -> Result<by_types::QueryResponse<AgitSummary>> {
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
        let items: Vec<AgitSummary> = AgitSummary::query_builder(user_id)
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
        AgitCreateRequest {
            title,
            description,
            external_link,
            banner_url,
            logo_url,
        }: AgitCreateRequest,
    ) -> Result<Agit> {
        if auth.is_none() {
            return Err(ServiceError::Unauthorized);
        }

        let agit = self
            .repo
            .insert(
                title,
                description,
                external_link,
                banner_url,
                logo_url,
                false,
            )
            .await?;

        Ok(agit)
    }

    async fn update(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: AgitUpdateRequest,
    ) -> Result<Agit> {
        if auth.is_none() {
            return Err(ServiceError::Unauthorized);
        }

        let agit = self.repo.update(id, param.into()).await?;
        Ok(agit)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<Agit> {
        if auth.is_none() {
            return Err(ServiceError::Unauthorized);
        }

        let agit = self.repo.delete(id).await?;
        Ok(agit)
    }
}
