pub trait ToDatabase<DbEntity> {
    fn to_db(self) -> DbEntity;
}

pub trait ToEntity<Entity> {
    fn to_entity(self) -> Entity;
}

pub trait FromDatabase<DbEntity> {
    fn from_db(entity: DbEntity) -> Self;
}

pub trait FromEntity<Entity> {
    fn from_entity(entity: Entity) -> Self;
}

impl<Entity, DbEntity> FromDatabase<DbEntity> for Entity
where
    DbEntity: ToEntity<Entity>,
{
    fn from_db(entity: DbEntity) -> Self {
        entity.to_entity()
    }
}

impl<Entity, DbEntity> ToDatabase<DbEntity> for Entity
where
    DbEntity: FromEntity<Entity>,
{
    fn to_db(self) -> DbEntity {
        DbEntity::from_entity(self)
    }
}

//impl<T> FromEntity<T> for T {
//    fn from_entity(entity: T) -> Self {
//        entity
//    }
//}
//
//impl<T> ToEntity<T> for T {
//    fn to_entity(self) -> T {
//        self
//    }
//}
