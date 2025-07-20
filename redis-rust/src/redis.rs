use crate::{error::AppError, kv_store::RedisKV};
use redis::{AsyncCommands, aio::ConnectionManager};
use std::future::Future;

pub struct RedisClient {
    connection_manager: ConnectionManager,
}

impl RedisClient {
    async fn new() -> Result<Self, AppError> {
        let redis_connection_url = std::env::var("REDIS_CONNECTION_URL")
            .map_err(|err| AppError::EnvError(err.to_string()))?;

        let redis_client = redis::Client::open(redis_connection_url).map_err(AppError::from)?;

        let connection_manager = ConnectionManager::new(redis_client)
            .await
            .map_err(AppError::from)?;

        Ok(Self { connection_manager })
    }
}

pub trait RedisClientExt {
    fn set(&mut self, key: &str, value: &str) -> impl Future<Output = Result<(), AppError>>;
    fn get(&mut self, key: &str) -> impl Future<Output = Result<RedisKV, AppError>>;
}

impl RedisClientExt for RedisClient {
    async fn set(&mut self, key: &str, value: &str) -> Result<(), AppError> {
        let _: () = self
            .connection_manager
            .set(key, value)
            .await
            .map_err(AppError::from)?;

        Ok(())
    }

    async fn get(&mut self, key: &str) -> Result<RedisKV, AppError> {
        let result: Option<String> = self.connection_manager.get(key).await.ok();

        if result.is_none() {
            return Err(AppError::ResourceNotFound);
        };

        Ok(RedisKV {
            key: key.to_string(),
            value: result.unwrap(),
        })
    }
}
