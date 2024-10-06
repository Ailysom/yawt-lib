use crate::{
    backend::{Backend, BackendConnector},
    yawt_object::{
        YawtError, YawtObject
    },
};

pub struct SqliteHandler {
    path_to_file: String,
    connection: rusqlite::Connection,
}

impl<T> Backend<T> for SqliteHandler
where
    T: YawtObject,
{
    // Implementation here
}

impl  BackendConnector for SqliteHandler {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init() {
        SqliteHandler::connect("test.db").unwrap();
    }
}