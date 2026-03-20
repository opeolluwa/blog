
use crate::errors::{auth_error::AuthenticationError, repository_error::RepositoryError};


#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("not found")]
    NotFound,
    #[error("conflict")]
    Conflict,
    #[error("unauthorized")]
    Unauthorized,
    #[error("internal server error")]
    InternalServerError,
    #[error(transparent)]
    RepositoryError(#[from] RepositoryError),
    #[error(transparent)]
    AuthenticationError(#[from] AuthenticationError),
    
}