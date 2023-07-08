use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SoEmployee::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SoEmployee::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SoEmployee::Title).string().not_null())
                    .col(ColumnDef::new(SoEmployee::Text).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(SoEmployee::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
/// 名字是蛇皮风格 so_employee
#[derive(Iden)]
enum SoEmployee {
    Table,
    Id,
    Title,
    Text,
}
