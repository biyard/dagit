use validator::ValidationErrors;

pub mod v1;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Error {
    #[error("Unknown error: {0}")]
    Unknown(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Service error: {0}")]
    DatabaseError(String),
    #[error("Reqwest error: {0}")]
    ReqwestError(String),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::ReqwestError(e.to_string())
    }
}

impl From<ValidationErrors> for Error {
    fn from(e: ValidationErrors) -> Self {
        Self::ValidationError(e.to_string())
    }
}

#[cfg(feature = "server")]
impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::DatabaseError(e.to_string())
    }
}

#[cfg(feature = "server")]
impl by_axum::axum::response::IntoResponse for Error {
    fn into_response(self) -> by_axum::axum::response::Response {
        (
            by_axum::axum::http::StatusCode::BAD_REQUEST,
            by_axum::axum::Json(self),
        )
            .into_response()
    }
}
