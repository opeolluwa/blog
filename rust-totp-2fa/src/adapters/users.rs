use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl Into<entities::users::ActiveModel> for CreateUserRequest {
    fn into(self) -> entities::users::ActiveModel {
        entities::users::ActiveModel {
            first_name: sea_orm::Set(self.first_name),
            last_name: sea_orm::Set(self.last_name),
            email: sea_orm::Set(self.email),
            password: sea_orm::Set(self.password),
            identifier: Set(Uuid::new_v4()),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
