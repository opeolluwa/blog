
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum RepositoryError {
    #[error("not found")]
    NotFound,
    #[error("database error")]
    DatabaseError,
}