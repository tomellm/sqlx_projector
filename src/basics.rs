use sqlx::{Database, QueryBuilder};

use crate::{
    projectors::FromEntity,
    DatabaseUtilities,
};

pub trait DatabaseBuilderExtensions<DB> {
    fn query(&mut self, query: &str) -> &mut Self;
    fn select(&mut self, cols: &str) -> &mut Self;
    fn from(&mut self, table: &str) -> &mut Self;
    fn insert_into(&mut self, table: &str) -> &mut Self;
    fn cols(&mut self, columns: &[&str]) -> &mut Self;
    fn value<Value, DbValue>(&mut self, value: Value) -> &mut Self
    where
        DbValue: FromEntity<Value> + DatabaseUtilities<DbValue, DB = DB>;
    fn delete_from(&mut self, table: &str) -> &mut Self;
}

impl<DB> DatabaseBuilderExtensions<DB> for QueryBuilder<'_, DB>
where
    DB: Database,
{
    /// simply an alias for push
    fn query(&mut self, query: &str) -> &mut Self {
        self.push(query)
    }
    /// adds select and then the passed string
    /// 'SELECT {cols} '
    fn select(&mut self, cols: &str) -> &mut Self {
        self.push(format!("SELECT {cols} "))
    }
    /// adds from and then the passed string
    /// 'FROM {table} '
    fn from(&mut self, table: &str) -> &mut Self {
        self.push(format!("FROM {table} "))
    }
    /// adds insert into and then the table name
    /// 'INSERT INTO {table} '
    fn insert_into(&mut self, table: &str) -> &mut Self {
        self.push(format!("INSERT INTO {table} "))
    }
    /// adds the columns in round brackets for insert into
    /// columns are comma separated
    /// '({col1}, {col2}, {col3}) '
    fn cols(&mut self, columns: &[&str]) -> &mut Self {
        self.push(format!("({}) ", columns.join(", ")))
    }
    /// Uses push_values to add a single value to the query. This will add both
    /// the 'VALUES' as well as the placeholders and attach the variables
    fn value<Value, DbValue>(&mut self, value: Value) -> &mut Self
    where
        DbValue: FromEntity<Value> + DatabaseUtilities<DbValue, DB = DB>,
    {
        self.push_values(vec![DbValue::from_entity(value)], DbValue::push_touple_fn())
    }
    fn delete_from(&mut self, table: &str) -> &mut Self {
        self.push(format!("DELETE FROM {table} "))
    }
}
