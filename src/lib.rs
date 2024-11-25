pub mod in_items;
pub mod projectors;
pub mod values;
pub mod basics;

use std::pin::Pin;

use futures::future::try_join_all;
use projectors::{FromDatabase, ToEntity};
use sqlx::{query_builder::Separated, Database, Error, QueryBuilder};

pub trait DatabaseUtilities<DbEntity> {
    type DB: Database;
    fn table_name() -> &'static str;
    fn column_names() -> &'static [&'static str];
    fn push_touple_fn() -> impl FnMut(Separated<'_, '_, Self::DB, &'static str>, DbEntity);
}

impl<Entity, DbEntity> DatabaseUtilities<DbEntity> for Entity
where
    Entity: FromDatabase<DbEntity>,
    DbEntity: ToEntity<DbEntity> + DatabaseUtilities<DbEntity>,
{
    type DB = DbEntity::DB;
    fn table_name() -> &'static str {
        DbEntity::table_name()
    }
    fn column_names() -> &'static [&'static str] {
        DbEntity::column_names()
    }
    fn push_touple_fn() -> impl FnMut(Separated<'_, '_, Self::DB, &'static str>, DbEntity) {
        DbEntity::push_touple_fn()
    }
}

pub fn builder<'args, DB>() -> QueryBuilder<'args, DB>
where
    DB: Database,
{
    QueryBuilder::new("")
}

pub trait AwaitQueryResponses<DB> {
    fn join_await(self) -> impl std::future::Future<Output = Result<(), ()>>;
}

impl<Fut, DB> AwaitQueryResponses<DB> for Vec<Fut>
where
    Fut:
        std::future::Future<Output = Result<<DB as Database>::QueryResult, Error>> + Send + 'static,
    DB: Database,
{
    fn join_await(self) -> impl std::future::Future<Output = Result<(), ()>> {
        Box::pin(async move {
            try_join_all(self).await.unwrap();
            Ok(())
        }) as Pin<Box<dyn std::future::Future<Output = Result<(), ()>> + Send>>
    }
}
