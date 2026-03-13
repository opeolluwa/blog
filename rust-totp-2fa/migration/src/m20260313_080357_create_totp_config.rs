use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("totp_settings")
                    .if_not_exists()
                    .col(uuid("identifier"))
                    .col(uuid("user_identifier"))
                    .col(binary("secret").binary_len(64))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_totp_identifier")
                            .from("totp_settings", "user_identifier")
                            .to("users", "identifier")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("totp_settings").to_owned())
            .await
    }
}
