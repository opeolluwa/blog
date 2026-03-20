
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthenticationError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("two-factor authentication required")]
    TwoFactorRequired,
    #[error("invalid token")]
    InvalidToken,
    #[error("token expired")]
    TokenExpired,
    #[error("token revoked")]
    TokenRevoked,
    #[error("internal server error")]
    InternalServerError,
    #[error(transparent)]
    AppError(#[from] crate::errors::app_error::AppError),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("{0}")]
    Other(String),
}
