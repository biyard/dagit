use serde::{Deserialize, Serialize};

use bdk::prelude::*;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Deserialize, Translate)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum ServiceError {
    //Common
    Unknown(String),
    NotFound,
    BadRequest(String),
    // Auth Errors
    Unauthorized,
    DuplicateUser,
    #[translate(
        ko = "인증코드가 만료되었습니다.",
        en = "Verification code is expired."
    )]
    AuthenticationCodeExpired,

    InternalServerError(String),
    ReqwestError(String),
    DatabaseError(String),
    ValidationError(String),
    JwtGenerationFailed(String),
    Unsupported(String),

    #[translate(
        ko = "회원 가입에 실패했습니다. 잠시 후 다시 시도해주세요.",
        en = "Failed to sign up. Please try again later."
    )]
    SignupFailed,
}

impl std::error::Error for ServiceError {}

impl From<reqwest::Error> for ServiceError {
    fn from(e: reqwest::Error) -> Self {
        ServiceError::ReqwestError(e.to_string())
    }
}

impl From<gloo_net::Error> for ServiceError {
    fn from(e: gloo_net::Error) -> Self {
        ServiceError::ReqwestError(e.to_string())
    }
}

impl From<validator::ValidationErrors> for ServiceError {
    fn from(e: validator::ValidationErrors) -> Self {
        ServiceError::ValidationError(e.to_string())
    }
}

#[cfg(feature = "server")]
impl From<sqlx::Error> for ServiceError {
    fn from(e: sqlx::Error) -> Self {
        ServiceError::DatabaseError(e.to_string())
    }
}

impl ServiceError {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[cfg(feature = "server")]
impl by_axum::axum::response::IntoResponse for ServiceError {
    fn into_response(self) -> by_axum::axum::response::Response {
        let status_code = match self {
            ServiceError::NotFound => by_axum::axum::http::StatusCode::NOT_FOUND,
            ServiceError::Unauthorized => by_axum::axum::http::StatusCode::UNAUTHORIZED,
            ServiceError::BadRequest(_) | ServiceError::ValidationError(_) => {
                by_axum::axum::http::StatusCode::BAD_REQUEST
            }
            ServiceError::InternalServerError(_) | ServiceError::DatabaseError(_) => {
                by_axum::axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            _ => by_axum::axum::http::StatusCode::BAD_REQUEST,
        };

        (status_code, by_axum::axum::Json(self)).into_response()
    }
}
