use crate::{
    backend::Backend,
    yawt_object::{
        YawtError, YawtObject
    },
};

pub struct SqliteHandler {
    path_to_file: String,
    connection: rusqlite::Connection,
}

impl Backend for SqliteHandler
{
    fn connect(connection_string: &str) -> Result<SqliteHandler, YawtError> {
        let con = match rusqlite::Connection::open(connection_string) {
            Ok(con) => con,
            Err(err) => return Err(YawtError::from_str(&err.to_string())),   
        };
        Ok(SqliteHandler {
            path_to_file: String::from(connection_string),
            connection: con,
        })
    }

    fn save<T: YawtObject>(&self, obj: &T) -> Result<(), YawtError> {
        let values = obj.to_string_array();
        let execute_string = format!(
            "INSERT INTO {} ({}) values ({}?)",
            T::get_storage_name(),
            T::get_positions_for_sql(),
            "?, ".repeat(values.len() - 1)
        );
        self.connection.execute(&execute_string, values);
        return Err(YawtError::from_str("not implemented yet"));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init() {
        SqliteHandler::connect("test.db").unwrap();
    }
}