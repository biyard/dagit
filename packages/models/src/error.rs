use std::error::Error;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;

#[derive(Debug, Serialize, PartialEq, Eq, Deserialize)]
#[repr(u64)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum ServiceError {
    Unknown(String),
    NotFound,
    Unauthorized,
    BadRequest(String),
    Conflict(String),
    InternalServerError(String),
    DatabaseError(String),
    ValidationError(String),
    Forbidden,
    CannotCreateAgit,
    CannotUpdateAgit,
    CannotDeleteAgit,
    AgitAlreadyExists,
    AgitNotFound,
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for ServiceError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ServiceError::Unknown(s.to_string()))
    }
}

impl<E: Error + 'static> From<E> for ServiceError {
    fn from(e: E) -> Self {
        ServiceError::Unknown(e.to_string())
    }
}

impl ServiceError {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

unsafe impl Send for ServiceError {}
unsafe impl Sync for ServiceError {}

#[cfg(feature = "server")]
impl by_axum::axum::response::IntoResponse for ServiceError {
    fn into_response(self) -> by_axum::axum::response::Response {
        let status_code = match self {
            ServiceError::NotFound | ServiceError::AgitNotFound => by_axum::axum::http::StatusCode::NOT_FOUND,
            ServiceError::Unauthorized => by_axum::axum::http::StatusCode::UNAUTHORIZED,
            ServiceError::Forbidden => by_axum::axum::http::StatusCode::FORBIDDEN,
            ServiceError::BadRequest(_) | ServiceError::ValidationError(_) => by_axum::axum::http::StatusCode::BAD_REQUEST,
            ServiceError::Conflict(_) | ServiceError::AgitAlreadyExists => by_axum::axum::http::StatusCode::CONFLICT,
            ServiceError::CannotCreateAgit | ServiceError::CannotUpdateAgit | ServiceError::CannotDeleteAgit => by_axum::axum::http::StatusCode::BAD_REQUEST,
            ServiceError::InternalServerError(_) | ServiceError::DatabaseError(_) => by_axum::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            _ => by_axum::axum::http::StatusCode::BAD_REQUEST,
        };

        (
            status_code,
            by_axum::axum::Json(self),
        )
            .into_response()
    }
}
