use uuid::Uuid;

use crate::YawtObject;

pub trait StateHandler<T>
where T: YawtObject {
    fn get(&self, id: Uuid) -> Result<T, crate::error::Error> ;
    fn save(&self, obj: &T) -> Result<(), crate::error::Error> ;
    fn delete(&self, obj: &T) -> Result<(), crate::error::Error> ;
}