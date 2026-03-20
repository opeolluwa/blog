use serde::{Deserialize, Serialize};

use crate::errors::repository_error::RepositoryError;


#[derive(Debug, Serialize, thiserror::Error, Deserialize)]
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
    
}