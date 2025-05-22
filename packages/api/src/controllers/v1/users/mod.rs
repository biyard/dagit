#[cfg(test)]
mod tests;

use bdk::prelude::{by_types::TokenScheme, *};
use by_axum::{
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
        routing::post,
    },
};
use by_types::JsonWithHeaders;
use common::error::ServiceError;
use common::tables::prelude::*;
use common::{Result, tables::user_terms::UserTerm};
use sqlx::postgres::PgRow;

use crate::utils::app_claims::AppClaims;

#[derive(Clone, Debug)]
pub struct UserController {
    pool: sqlx::PgPool,
    repo: UserRepository,
}

impl UserController {
    pub fn new(pool: sqlx::PgPool) -> Self {
        let repo = User::get_repository(pool.clone());
        Self { repo, pool }
    }
    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act).get(Self::get))
            .with_state(self.clone()))
    }
}

impl UserController {
    pub async fn act(
        State(ctrl): State<UserController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<UserAction>,
    ) -> Result<by_types::JsonWithHeaders<User>> {
        match body {
            UserAction::Signup(req) => ctrl.signup(req).await,
            UserAction::Login(req) => ctrl.login(req).await,
            UserAction::UpdateProfile(req) => ctrl.update_profile(req, auth).await,
        }
    }

    pub async fn get(
        State(ctrl): State<UserController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<UserParam>,
    ) -> Result<JsonWithHeaders<User>> {
        tracing::debug!("act {:?}", q);

        match q {
            UserParam::Read(param)
                if param.action == Some(UserReadActionType::GetUserByAddress) =>
            {
                let user = ctrl.get_user_by_address(param).await?;
                Ok(JsonWithHeaders::new(user))
            }
            UserParam::Read(action) => match action.action.unwrap() {
                UserReadActionType::Refresh => {
                    if auth.is_none() {
                        return Err(ServiceError::Unauthorized);
                    }
                    ctrl.refresh_user(auth.unwrap()).await
                }
                _ => Err(ServiceError::BadRequest("Invalid action".to_string())),
            },
            _ => {
                unimplemented!()
            }
        }
    }
}

impl UserController {
    async fn refresh_user(&self, auth: Authorization) -> Result<JsonWithHeaders<User>> {
        let user_id = match auth {
            Authorization::Bearer { ref claims } => AppClaims(claims).get_id(),
            _ => return Err(ServiceError::Unauthorized),
        };
        let user = User::query_builder()
            .id_equals(user_id)
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&self.pool)
            .await?;
        let jwt = AppClaims::generate_token(&user)?;
        Ok(JsonWithHeaders::new(user).with_auth_cookie(TokenScheme::Bearer, &jwt))
    }

    async fn get_user_by_address(
        &self,
        UserReadAction { address, .. }: UserReadAction,
    ) -> Result<User> {
        let user: User = User::query_builder()
            .address_equals(address.unwrap_or_default())
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn signup(
        &self,
        UserSignupRequest {
            address,
            name,
            email,
            profile_url,
            provider,
            terms_agreed_at,
            ads_agreed_at,
            agit_title,
        }: UserSignupRequest,
    ) -> Result<JsonWithHeaders<User>> {
        let user_terms_repo = UserTerm::get_repository(self.pool.clone());
        let mut tx = self.pool.begin().await?;
        let agit = Agit::get_repository(self.pool.clone());
        let agit_admin = AgitAdmin::get_repository(self.pool.clone());
        let mut user = self
            .repo
            .insert_with_tx(&mut *tx, provider, address, email, name, profile_url)
            .await?
            .ok_or(ServiceError::DuplicateUser)?;

        user_terms_repo
            .insert_with_tx(&mut *tx, user.id, terms_agreed_at, ads_agreed_at)
            .await?;
        let agit = agit
            .insert_with_tx(
                &mut *tx,
                agit_title,
                String::default(),
                None,
                String::default(),
                String::default(),
                false,
            )
            .await?
            .ok_or(ServiceError::DatabaseError(
                "Create Agit Failed".to_string(),
            ))?;

        agit_admin
            .insert_with_tx(&mut *tx, agit.id, user.id)
            .await?;

        tx.commit().await?;
        user.agits = vec![agit];
        let jwt = AppClaims::generate_token(&user)?;
        Ok(JsonWithHeaders::new(user).with_auth_cookie(TokenScheme::Bearer, &jwt))
    }

    async fn login(
        &self,
        UserLoginRequest { address, provider }: UserLoginRequest,
    ) -> Result<JsonWithHeaders<User>> {
        let user = User::query_builder()
            .address_equals(address)
            .provider_equals(provider)
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&self.pool)
            .await?;

        let jwt = AppClaims::generate_token(&user)?;
        Ok(JsonWithHeaders::new(user).with_auth_cookie(TokenScheme::Bearer, &jwt))
    }

    async fn update_profile(
        &self,
        req: UserUpdateProfileRequest,
        auth: Option<Authorization>,
    ) -> Result<JsonWithHeaders<User>> {
        let principal = match auth {
            Some(Authorization::Bearer { claims }) => claims.sub,
            _ => {
                return Err(common::error::ServiceError::Unauthorized);
            }
        };

        let mut repo_req = UserRepositoryUpdateRequest::new().with_name(req.name);
        if req.profile_url.is_some() {
            repo_req = repo_req.with_profile_url(req.profile_url.unwrap());
        }

        let user = self
            .repo
            .find_one(&UserReadAction::new().get_user_by_address(principal))
            .await?;

        let user = self.repo.update(user.id, repo_req).await?;
        Ok(JsonWithHeaders::new(user))
    }
}
