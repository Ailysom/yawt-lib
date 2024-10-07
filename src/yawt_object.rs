use {
    core::fmt,
    std::rc::Rc,
    serde_json::json
};

pub trait YawtObject
where Self: serde::Serialize + Sized {
	fn to_json(&self) -> String {
        json!(self).to_string()
    }
    fn from_sqlite_row<'a>(row: &rusqlite::Row) -> Result<Self, YawtError> {
        return Err(YawtError::from_str("not implemented yet"));
    }
    fn to_string_array(&self) -> [String; 5]; //TODO: Change 5, to usize
    fn get_storage_name() -> &'static str;
    fn get_positions_for_sql() -> &'static str;
}

#[derive(Debug)]
pub struct YawtError {
    message: Rc<str>,
}

impl<'a> YawtError {
    pub fn from_str(msg: &str) -> Self {
        YawtError { message:Rc::from(msg) }
    }
    pub fn set_message(&'a mut self, msg: &'a str) {
        self.message = Rc::from(msg);
    }
    pub fn to_str(&self) -> &str {
        &self.message
    }
}

impl<'a> fmt::Display for YawtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}