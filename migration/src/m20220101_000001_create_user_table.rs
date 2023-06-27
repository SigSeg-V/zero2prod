use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            //.auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::CreatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await

        // manager
        //     .create_table(
        //         Table::create()
        //             .table(Post::Table)
        //             .if_not_exists()
        //             .col(
        //                 ColumnDef::new(Post::Id)
        //                     .integer()
        //                     .not_null()
        //                     .auto_increment()
        //                     .primary_key(),
        //             )
        //             .col(ColumnDef::new(Post::Title).string().not_null())
        //             .col(ColumnDef::new(Post::Text).string().not_null())
        //             .to_owned(),
        //     )
        //     .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    Id,
    Email,
    Name,
    CreatedAt,
}
