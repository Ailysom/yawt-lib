use {
    crate::yawt_object::YawtObject,
    uuid::Uuid,
};

pub trait Backend<T>
where T: YawtObject {
    fn get(&self, id: Uuid) -> Result<T, &str>;
    fn save(&self, obj: &T) -> Result<(), &str>;
    fn delete(&self, obj: &T) -> Result<(), &str>;
}