pub mod basics;
pub mod in_items;
pub mod projectors;
pub mod values;

use std::pin::Pin;

use futures::future::try_join_all;
use projectors::{FromEntity, ToDatabase, ToEntity};
use sqlx::{query_builder::Separated, Database, Error, QueryBuilder};

pub trait DatabaseUtilities<DbEntity> {
    type DB: Database;
    fn table_name() -> &'static str;
    fn column_names() -> &'static [&'static str];
    fn push_touple_fn() -> impl FnMut(Separated<'_, '_, Self::DB, &'static str>, DbEntity);
}

impl<Entity, DbEntity> DatabaseUtilities<DbEntity> for Entity
where
    Entity: ToDatabase<DbEntity = DbEntity>,
    DbEntity: DatabaseUtilities<DbEntity> + FromEntity<Self> + ToEntity<Self>,
{
    type DB = <DbEntity as DatabaseUtilities<DbEntity>>::DB;
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

#[cfg(test)]
mod test {
    use sqlx::{query_builder::Separated, Postgres};
    use static_assertions::assert_impl_one;

    use crate::{
        impl_to_database, projectors::{FromEntity, ToDatabase, ToEntity}, DatabaseUtilities
    };

    #[test]
    fn database_util_implemented() {
        assert_impl_one!(Struct: DatabaseUtilities<DbStruct>);
        assert_eq!(Struct::table_name(), "table_name");

        assert_impl_one!(OtherStruct: DatabaseUtilities<DbOtherStruct>);
        assert_eq!(OtherStruct::table_name(), "other_table_name");
    }

    #[test]
    fn to_database_implemented() {
        assert_impl_one!(Struct: ToDatabase<DbEntity = DbStruct>);
        assert_eq!(Struct.to_db(), DbStruct);
        assert_impl_one!(OtherStruct: ToDatabase<DbEntity = DbOtherStruct>);
        assert_eq!(OtherStruct.to_db(), DbOtherStruct);
    }

    #[derive(Clone, Copy, Eq, PartialEq, Debug)]
    struct DbStruct;
    #[derive(Clone, Copy, Eq, PartialEq, Debug)]
    struct Struct;

    #[derive(Clone, Copy, Eq, PartialEq, Debug)]
    struct DbOtherStruct;
    #[derive(Clone, Copy, Eq, PartialEq, Debug)]
    struct OtherStruct;

    impl ToEntity<Struct> for DbStruct {
        fn to_entity(self) -> Struct {
            Struct
        }
    }

    impl FromEntity<Struct> for DbStruct {
        fn from_entity(_: Struct) -> Self {
            DbStruct
        }
    }

    impl_to_database!(Struct, DbStruct);

    impl ToEntity<OtherStruct> for DbOtherStruct {
        fn to_entity(self) -> OtherStruct {
            OtherStruct
        }
    }

    impl FromEntity<OtherStruct> for DbOtherStruct {
        fn from_entity(_: OtherStruct) -> Self {
            DbOtherStruct
        }
    }

    impl_to_database!(OtherStruct, DbOtherStruct);

    impl DatabaseUtilities<DbStruct> for DbStruct {
        type DB = Postgres;
        fn table_name() -> &'static str {
            "table_name"
        }
        fn column_names() -> &'static [&'static str] {
            &["col_one", "col_two"]
        }
        fn push_touple_fn() -> impl FnMut(Separated<'_, '_, Self::DB, &'static str>, DbStruct) {
            |_builder, _value| {}
        }
    }

    impl DatabaseUtilities<DbOtherStruct> for DbOtherStruct {
        type DB = Postgres;
        fn table_name() -> &'static str {
            "other_table_name"
        }
        fn column_names() -> &'static [&'static str] {
            &["other_col_one", "other_col_two"]
        }
        fn push_touple_fn() -> impl FnMut(Separated<'_, '_, Self::DB, &'static str>, DbOtherStruct)
        {
            |_builder, _value| {}
        }
    }
}
