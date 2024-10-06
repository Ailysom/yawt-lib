use {
    crate::yawt_object::{
        YawtObject,
        YawtError,
    },
    uuid::Uuid,
};

pub trait Backend
where
    Self: Sized,
{
    fn get<T: YawtObject>(&self, id: Uuid) -> Result<T, YawtError> {
        return Err(YawtError::from_str("not implemented yet"));
    }
    fn save<T: YawtObject>(&self, obj: &T) -> Result<(), YawtError> {
        return Err(YawtError::from_str("not implemented yet"));
    }
    fn delete<T: YawtObject>(&self, obj: &T) -> Result<(), YawtError> {
        return Err(YawtError::from_str("not implemented yet"));
    }
    fn connect(connection_string: &str) -> Result<Self, YawtError>  {
        return Err(YawtError::from_str("not implemented yet"));
    }
}