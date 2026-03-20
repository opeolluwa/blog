use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::entities::totp_settings::Model;
use crate::entities::prelude::TotpSettings;
use crate::errors::repository_error::RepositoryError;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

pub struct TotpSettingsRepository{
    db_conn: Arc<DatabaseConnection>
}

pub trait TotpSettingsRepositoryExt {
    async fn get_by_user_id(&self, user_id: &str) -> Result<TotpSettings, RepositoryError>;
    async fn create(&self, settings: &TotpSettings) -> Result<(), RepositoryError>;
    async fn update(&self, settings: &TotpSettings) -> Result<(), RepositoryError>;
    async fn delete(&self, user_id: &str) -> Result<(), RepositoryError>;
}

impl TotpSettingsRepositoryExt for TotpSettingsRepository {
    async fn get_by_user_id(&self, user_id: &str) -> Result<TotpSettings, RepositoryError> {
        let settings = TotpSettings::find()
            .filter(TotpSettings::Column::UserIdentifier.eq(user_id))
            .one(&self.db_conn.as_ref())
            .await?;
        settings.ok_or(RepositoryError::NotFound)
    }

    async fn create(&self, settings: &TotpSettings) -> Result<(), RepositoryError> {

    }

    async fn update(&self, settings: &TotpSettings) -> Result<(), RepositoryError> {
        
    }

    async fn delete(&self, user_id: &str) -> Result<(), RepositoryError> {
        
    }
}
