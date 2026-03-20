
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("environment variable error: {0}")]
    EnvError(String),
    
}
