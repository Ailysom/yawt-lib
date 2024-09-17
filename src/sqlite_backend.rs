use {
    crate::{
        backend::Backend, task::Task, yawt_object::YawtObject
    }, chrono::{DateTime, Utc, NaiveDateTime}, rusqlite::{Connection, Result}, serde_json::from_str, std::str::FromStr, uuid::Uuid
};

struct SqliteHandler {}

impl<T> Backend<T> for SqliteHandler 
where T: YawtObject {
    fn get(&self, id: Uuid) -> Result<T, &str> {
        let conn = Connection::open("test.db").unwrap();
        let mut stmt = conn.prepare(
            "SELECT * FROM task;",
        ).unwrap();
        let result: T = stmt.query_row([], |row| { Ok(crate::yawt_object::YawtObject::from_sqlite_row(row))}).unwrap();
        Ok(result)
    }
    fn save(&self, obj: &T) -> Result<(), &str> {
        Err("not implemented")
    }
    fn delete(&self, obj: &T) -> Result<(), &str> {
        Err("not implemented")
    }
}

#[cfg(test)]
mod tests {

use std::str::FromStr;

use crate::{task::Task, yawt_object::YawtObject};

use super::*;

	#[test]
	fn get_task_from_sqlite() {
        let handler = SqliteHandler{};
        let from_storage: Task = handler
            .get(Uuid::from_str("67e55044-10b1-426f-9247-bb680e5fe0c8")
                .unwrap()
            ).unwrap();
         // because by default deadline eq timestamp & it's public
        let now = from_storage.time_stamp;
        let result = format!(
            "{{\"deadline\":\"{}\",\"description\":\"\",\"id\":\"{}\",\"priority\":5,\"time_stamp\":\"{}\"}}",
            now,
            "67e55044-10b1-426f-9247-bb680e5fe0c8",
            now,
        );
		assert_eq!(result, from_storage.to_json());
	}
}