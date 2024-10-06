use {
    crate::yawt_object::{
        YawtObject,
        YawtError,
    },
    uuid::Uuid,
};

pub trait Backend<T>
where
    Self: Sized,
    T: YawtObject,
{
    fn get(&self, id: Uuid) -> Result<T, YawtError> {
        return Err(YawtError::from_str("not implemented yet"));
    }
    fn save(&self, obj: &T) -> Result<(), YawtError> {
        return Err(YawtError::from_str("not implemented yet"));
    }
    fn delete(&self, obj: &T) -> Result<(), YawtError> {
        return Err(YawtError::from_str("not implemented yet"));
    }
}

pub trait BackendConnector: Sized {
    fn connect(connection_string: &str) -> Result<Self, YawtError>  {
        return Err(YawtError::from_str("not implemented yet"));
    }
}