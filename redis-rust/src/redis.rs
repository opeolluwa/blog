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
    fn get(&mut self, key: &str, value: &str) -> impl Future<Output = Result<RedisKV, AppError>>;
    fn set(&mut self, key: &str) -> impl Future<Output = Result<RedisKV, AppError>>;
}

impl RedisClientExt for RedisClient {
    async fn get(&mut self, key: &str, value: &str) -> Result<RedisKV, AppError> {
        let result: Option<String> = self.connection_manager.get(key).await.ok();
        todo!()
    }

    async fn set(&mut self, key: &str) -> Result<RedisKV, AppError> {
        todo!()
    }
}
