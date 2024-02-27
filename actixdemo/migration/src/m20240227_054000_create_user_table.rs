use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AtxUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AtxUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AtxUser::Nickname).string().not_null())
                    .col(ColumnDef::new(AtxUser::Age).integer().not_null())
                    .col(ColumnDef::new(AtxUser::CreateAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AtxUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AtxUser {
    Table,
    Id,
    Nickname,
    Age,
    CreateAt,
}
