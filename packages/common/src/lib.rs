pub mod error;
pub mod tables;
pub type Result<T> = std::result::Result<T, error::ServiceError>;
