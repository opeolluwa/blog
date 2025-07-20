use redis::RedisError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("unable to extract ENV due to {0}")]
    EnvError(String),
    #[error("resource not found")]
    ResourceNotFound,
    #[error(transparent)]
    RedisError(#[from] RedisError),
}
