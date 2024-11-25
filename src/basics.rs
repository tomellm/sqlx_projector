use sqlx::{Database, QueryBuilder};

pub trait DatabaseBuilderExtensions {
    fn query(&mut self, query: &str) -> &mut Self;
    fn select(&mut self, cols: &str) -> &mut Self;
    fn from(&mut self, table: &str) -> &mut Self;
}

impl<DB> DatabaseBuilderExtensions for QueryBuilder<'_, DB>
where
    DB: Database
{
    fn query(&mut self, query: &str) -> &mut Self {
        self.push(query)
    }
    fn select(&mut self, cols: &str) -> &mut Self {
        self.push(format!("SELECT {cols} "))
    }

    fn from(&mut self, table: &str) -> &mut Self {
        self.push(format!("FROM {table} "))
    }
}
