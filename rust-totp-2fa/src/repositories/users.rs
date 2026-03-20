use crate::entities;
use crate::entities::users::Model as Users;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use std::sync::Arc;
use uuid::Uuid;
use crate::errors::repository_error::RepositoryError;
use crate::adapters::users::CreateUserRequest;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

#[derive(Debug, Clone)]
pub struct UserRepository {
    db_conn: Arc<DatabaseConnection>,
}

impl UserRepository {
    pub fn new(db_conn: Arc<DatabaseConnection>) -> Self {
        Self { db_conn }
    }
}

pub(crate) trait UserRepositoryExt {

    async fn find_by_email(&self, email: &str) -> Option<Users>;

    async fn find_by_identifier(&self, identifier: Uuid) -> Option<Users>;

    async fn create(&self, req: CreateUserRequest) -> Result<Users, RepositoryError>;
}


impl UserRepositoryExt for UserRepository {
    async fn create(&self, req: CreateUserRequest) -> Result<Users, RepositoryError> {
        let active_model : entities::users::ActiveModel = req.into();
        let result = active_model.insert(self.db_conn.as_ref()).await.map_err(|_| RepositoryError::DatabaseError)?;
        Ok(result.into())
    }

    async fn find_by_email(&self, email: &str) -> Option<Users> {
        entities::users::Entity::find()
            .filter(entities::users::Column::Email.eq(email))
            .one(self.db_conn.as_ref())
            .await
            .ok()
            .flatten()

    }

    async fn find_by_identifier(&self, identifier: Uuid) -> Option<Users> {
        entities::users::Entity::find()
            .filter(entities::users::Column::Identifier.eq(identifier))
            .one(self.db_conn.as_ref())
            .await
            .ok()
            .flatten()
    }
}
