pub mod error;
pub mod v1;

pub type Result<T> = std::result::Result<T, error::ServiceError>;
