use crate::{state_handler, task::Task,};
use uuid::Uuid;
use diesel::prelude::*;
use crate::schema::task::dsl::task;
struct SqliteHandler {}

impl<T> state_handler::StateHandler<T> for SqliteHandler 
where T: super::YawtObject {
    fn get(&self, id: Uuid) -> Result<T, crate::error::Error> {
        let conn = &mut SqliteConnection::establish("./test.db").unwrap();
        let result: Vec<Task> = task
            .select(Task::as_select())
            .load(conn)
            .expect("Eror loading task");
        
        Err(crate::error::Error {
            message: "not implemented",
        })
    }
    fn save(&self, obj: &T) -> Result<(), crate::error::Error> {
        Err(crate::error::Error {
            message: "not implemented",
        })
    }
    fn delete(&self, obj: &T) -> Result<(), crate::error::Error> {
        Err(crate::error::Error {
            message: "not implemented",
        })
    }
}

#[cfg(test)]
mod tests {

use std::str::FromStr;

use state_handler::StateHandler;

use crate::{task::Task, YawtObject};

use super::*;

	#[test]
	fn get_task_from_sqlite() {
        let handler = SqliteHandler{};
        let from_storage: Task = handler
            .get(Uuid::from_str("67e55044-10b1-426f-9247-bb680e5fe0c8")
                .unwrap()
            ).unwrap();
         // because by default deadline eq timestamp & it's public
        let now = "";
        // from_storage.deadline.to_rfc3339_opts(
        //     chrono::SecondsFormat::Nanos,
        //     true
        // );
        let result = format!(
            "{{\"deadline\":\"{}\",\"description\":\"\",\"id\":\"{}\",\"priority\":5,\"time_stamp\":\"{}\"}}",
            now,
            "67e55044-10b1-426f-9247-bb680e5fe0c8",
            now,
        );
		assert_eq!(result, from_storage.to_json());
	}
}