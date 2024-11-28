use sqlx::QueryBuilder;

use crate::{projectors::FromEntity, DatabaseUtilities};

const BIND_LIMIT: usize = 10000;

pub trait SaveManyQuery<'args, Entity, DbEntity>
where
    Entity: DatabaseUtilities<DbEntity>,
    DbEntity: DatabaseUtilities<DbEntity> + FromEntity<Entity>,
{
    fn many_values(
        &self,
        data: Vec<Entity>,
    ) -> Vec<QueryBuilder<'args, <DbEntity as DatabaseUtilities<DbEntity>>::DB>>;
}

impl<'args, Entity, DbEntity> SaveManyQuery<'args, Entity, DbEntity>
    for QueryBuilder<'args, <DbEntity as DatabaseUtilities<DbEntity>>::DB>
where
    Entity: DatabaseUtilities<DbEntity>,
    DbEntity: DatabaseUtilities<DbEntity> + FromEntity<Entity>,
{
    fn many_values(
        &self,
        data: Vec<Entity>,
    ) -> Vec<QueryBuilder<'args, <DbEntity as DatabaseUtilities<DbEntity>>::DB>> {
        let block_length: usize = BIND_LIMIT / Entity::column_names().len();
        if data.is_empty() {
            return vec![];
        }

        data.into_iter()
            .enumerate()
            .fold(
                vec![],
                |mut acc: Vec<(
                    QueryBuilder<<DbEntity as DatabaseUtilities<DbEntity>>::DB>,
                    Vec<DbEntity>,
                )>,
                 (pos, data): (usize, Entity)| {
                    let index = (pos as f32 / block_length as f32).floor() as usize;
                    let inner_index = (pos as f32 % block_length as f32) as usize;
                    match acc.get_mut(index) {
                        Some(inner_vec) => {
                            inner_vec.1.insert(inner_index, DbEntity::from_entity(data));
                        }
                        None => {
                            //let query_str = format!(
                            //    "insert into {} ({})",
                            //    Entity::table_name(),
                            //    Entity::column_names().join(", ")
                            //);
                            let query_str = self.sql();
                            acc.insert(index, (QueryBuilder::new(query_str), vec![]));
                            let inner_vec = acc.get_mut(index).unwrap();
                            inner_vec.1.insert(inner_index, DbEntity::from_entity(data));
                        }
                    }
                    acc
                },
            )
            .into_iter()
            .map(|(mut builder, chunk)| {
                builder.push_values(chunk, DbEntity::push_touple_fn());
                builder
            })
            .collect()
    }
}
