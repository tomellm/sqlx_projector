pub trait ToDatabase {
    type DbEntity: FromEntity<Self> + ToEntity<Self>;
    fn to_db(self) -> Self::DbEntity;
}

#[macro_export]
macro_rules! impl_to_database {
    ($type:ty, $dbtype:ty) => {
        impl ToDatabase for $type {
            type DbEntity = $dbtype;
            fn to_db(self) -> Self::DbEntity {
                Self::DbEntity::from_entity(self)
            }
        }
    };
}

pub trait ToEntity<Entity>
where
    Entity: ?Sized,
{
    fn to_entity(self) -> Entity;
}

pub trait FromEntity<Entity>
where
    Entity: ?Sized,
{
    fn from_entity(entity: Entity) -> Self;
}
