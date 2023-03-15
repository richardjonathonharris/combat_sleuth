use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230310_000001_create_monster_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Monster::Table)
                    .col(
                        ColumnDef::new(Monster::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Monster::Name).string().not_null())
                    .col(ColumnDef::new(Monster::Hp).string().integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Monster::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Monster {
    Table,
    Id,
    Name,
    Hp,
}