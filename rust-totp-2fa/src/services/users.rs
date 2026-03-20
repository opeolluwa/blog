use crate::adapters::jwt::Claims;
use crate::adapters::users::{CreateUserRequest, LoginRequest, LoginResponse, SignUpRequest};
use crate::repositories::users::{UserRepository, UserRepositoryExt};
use crate::{entities, errors::service_error::ServiceError};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct UsersService {
    user_repo: UserRepository,
}

impl UsersService {
    pub fn new(db_conn: Arc<DatabaseConnection>) -> Self {
        Self {
            user_repo: UserRepository::new(db_conn),
        }
    }
}

pub trait UsersServiceExt {
    async fn find_by_email(&self, email: &str) -> Option<entities::users::Model>;
    async fn sign_up(&self, req: SignUpRequest) -> Result<(), ServiceError>;
    async fn login(&self, req: LoginRequest) -> Result<LoginResponse, ServiceError>;
    async fn validate_password(&self, password: &str, hash: &str) -> Result<bool, ServiceError>;
}

impl UsersServiceExt for UsersService {
    async fn find_by_email(&self, email: &str) -> Option<entities::users::Model> {
        self.user_repo.find_by_email(email).await
    }

    async fn sign_up(&self, req: SignUpRequest) -> Result<(), ServiceError> {
        let password_hash = bcrypt::hash(req.password, bcrypt::DEFAULT_COST).map_err(|err| {
            log::error!("{}", err.to_string());
            ServiceError::InternalServerError
        })?;

        let user = CreateUserRequest {
            first_name: req.first_name,
            last_name: req.last_name,
            email: req.email,
            password: password_hash,
        };
        self.user_repo.create(user).await?;
        Ok(())
    }

    async fn login(&self, req: LoginRequest) -> Result<LoginResponse, ServiceError> {
        let user = self
            .user_repo
            .find_by_email(&req.email)
            .await
            .ok_or(ServiceError::NotFound)?;

        let password_valid = self
            .validate_password(&req.password, &user.password)
            .await?;
        if !password_valid {
            return Err(ServiceError::Unauthorized);
        }

        let token = Claims::builder()
            .email(&user.email)
            .user_identifier(&user.identifier)
            .build_and_sign()?;

        Ok(LoginResponse { token })
    }

    async fn validate_password(&self, password: &str, hash: &str) -> Result<bool, ServiceError> {
        let password_valid = bcrypt::verify(&password, &hash).map_err(|err| {
            log::error!("{}", err.to_string());
            ServiceError::InternalServerError
        })?;

        Ok(password_valid)
    }
}
