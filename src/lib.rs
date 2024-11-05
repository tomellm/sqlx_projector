use std::{pin::Pin, sync::Arc};

use futures::future::try_join_all;
use sqlx::{query_builder::Separated, Database, Encode, Error, Executor, IntoArguments, Pool, QueryBuilder, Type};

const BIND_LIMIT: usize = 10000;

pub trait ToDatabase<DbEntity> {
    fn to_db(self) -> DbEntity;
}

pub trait FromDatabase<DbEntity> {
    fn from_db(entity: DbEntity) -> Self;
}

pub trait DatabaseUtilities<DB> {
    fn db_table_name() -> &'static str;
    fn db_column_names() -> &'static [&'static str];
    fn db_push_touple_fn() -> impl FnMut(Separated<'_, '_, DB, &'static str>, Self);
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

pub fn add_in_items<'args, DB, I, Entity, DbEntity>(
    query_front: &str,
    items: I,
    query_back: &str,
) -> QueryBuilder<'args, DB>
where
    I: IntoIterator<Item = Entity>,
    Entity: ToDatabase<DbEntity>,
    DbEntity: 'args + Encode<'args, DB> + Send + Type<DB>,
    DB: Database,
{
    let mut query_builder: QueryBuilder<'args, DB> = QueryBuilder::new(query_front);

    items
        .into_iter()
        .map(ToDatabase::to_db)
        .enumerate()
        .for_each(|(index, entity)| {
            if index != 0 {
                query_builder.push(",");
            };
            query_builder.push_bind(entity);
        });

    query_builder.push(query_back);

    query_builder
}

//pub fn save_many<DB, T>(
//    pool: Arc<Pool<DB>>,
//    t_data: Vec<T>,
//) -> impl std::future::Future<Output = Result<(), ()>>
//where
//    DB: Database,
//    for<'executor> &'executor mut <DB as Database>::Connection: Executor<'executor>,
//    for<'args> <DB as Database>::Arguments<'args>: IntoArguments<'args, DB>,
//    T: DatabaseUtilities<DB> + Send + 'static,
//{
//    let block_length: usize = BIND_LIMIT / T::db_column_names().len();
//    async move {
//        if t_data.is_empty() {
//            return Ok(());
//        }
//
//        let chunks = t_data.into_iter().enumerate().fold(
//            vec![],
//            |mut acc: Vec<(QueryBuilder<DB>, Vec<T>)>, (pos, data): (usize, T)| {
//                let index = (pos as f32 / block_length as f32).floor() as usize;
//                let inner_index = (pos as f32 % block_length as f32) as usize;
//                match acc.get_mut(index) {
//                    Some(inner_vec) => {
//                        inner_vec.1.insert(inner_index, data);
//                    }
//                    None => {
//                        let query_str = format!(
//                            "insert into {} ({})",
//                            T::db_table_name(),
//                            T::db_column_names().join(", ")
//                        );
//                        acc.insert(index, (QueryBuilder::new(query_str), vec![]));
//                        let inner_vec = acc.get_mut(index).unwrap();
//                        inner_vec.1.insert(inner_index, data);
//                    }
//                }
//                acc
//            },
//        );
//        let mut futures = vec![];
//        for (mut query_builder, chunk) in chunks.into_iter() {
//            query_builder.push_values(chunk, T::db_push_touple_fn());
//
//            let execute_pool = pool.clone();
//            futures.push(async move {
//                let query = query_builder.build();
//                query.execute(&*execute_pool).await
//            });
//        }
//
//        futures.join_await().await
//    }
//}
