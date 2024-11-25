use sqlx::{Database, Encode, QueryBuilder, Type};

pub trait PushInItems<'args, DB, Item>
where
    DB: Database,
    Item: 'args + Encode<'args, DB> + Send + Type<DB>,
{
    /// For the values ["a", "b", "c"] it will exactly generate the following
    /// string.
    ///
    /// " in (a, b, c) "
    ///  
    /// With the values added through the "push_bind" function to clean the input
    fn in_items<I>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = Item>;
}

pub trait PushExtractedInItems<'args, DB, Item, Entity>
where
    DB: Database,
    Item: 'args + Encode<'args, DB> + Send + Type<DB>,
    Entity: 'args,
{
    /// For the values ["a", "b", "c"] it will exactly generate the following
    /// string.
    ///
    /// " in (a, b, c) "
    ///  
    /// With the values added through the "push_bind" function to clean the input
    fn in_items_fn<I, F>(&'args mut self, entities: I, extract_fn: F) -> &'args mut Self
    where
        I: IntoIterator<Item = Entity>,
        F: Fn(Entity) -> Item;
}

impl<'args, DB, Item> PushInItems<'args, DB, Item> for QueryBuilder<'args, DB>
where
    DB: Database,
    Item: 'args + Encode<'args, DB> + Send + Type<DB>,
{
    fn in_items<I>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = Item>,
    {
        self.push(" in (");
        items.into_iter().enumerate().for_each(|(index, item)| {
            build_fn(self, index, item);
        });
        self.push(") ")
    }
}

impl<'args, DB, Item, Entity> PushExtractedInItems<'args, DB, Item, Entity>
    for QueryBuilder<'args, DB>
where
    DB: Database,
    Item: 'args + Encode<'args, DB> + Send + Type<DB>,
    Entity: 'args,
{
    fn in_items_fn<I, F>(&'args mut self, entities: I, extract_fn: F) -> &'args mut Self
    where
        I: IntoIterator<Item = Entity>,
        F: Fn(Entity) -> Item,
    {
        self.push("in (");
        entities
            .into_iter()
            .map(extract_fn)
            .enumerate()
            .for_each(|(index, item)| {
                build_fn(self, index, item);
            });
        self.push(") ")
    }
}

fn build_fn<'args, DB, Item>(builder: &mut QueryBuilder<'args, DB>, index: usize, item: Item)
where
    DB: Database,
    Item: 'args + Encode<'args, DB> + Send + Type<DB>,
{
    if index != 0 {
        builder.push(", ");
    };
    builder.push_bind(item);
}

#[cfg(test)]
mod test {
    use sqlx::{Execute, Postgres};

    use crate::{builder, in_items::PushExtractedInItems};

    use super::PushInItems;

    #[test]
    fn add_in_items() {
        let names = ["tom", "jojo", "jeff"];

        let mut query_builer = builder::<Postgres>();

        let query = query_builer
            .push("SELECT * FROM name WHERE name")
            .in_items(names)
            .build();

        assert_eq!(
            query.sql(),
            "SELECT * FROM name WHERE name in ($1, $2, $3) "
        );
    }

    #[test]
    fn add_in_items_fn() {
        struct Entity<'a> {
            name: &'a str,
            _age: usize,
        }

        let people = [
            Entity {
                name: "tom",
                _age: 23,
            },
            Entity {
                name: "jojo",
                _age: 43,
            },
            Entity {
                name: "jeff",
                _age: 13,
            },
        ];

        fn extract_fn(person: Entity<'_>) -> &str {
            person.name
        }

        let mut query_builer = builder::<Postgres>();

        let query = query_builer
            .push("SELECT * FROM name WHERE name ")
            .in_items_fn(people, extract_fn)
            .build();

        assert_eq!(
            query.sql(),
            "SELECT * FROM name WHERE name in ($1, $2, $3);"
        );
    }
}
