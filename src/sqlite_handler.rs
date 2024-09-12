use crate::state_handler;
use uuid::Uuid;

struct SqliteHandler {}

impl<T> state_handler::StateHandler<T> for SqliteHandler 
where T: super::YawtObject {
    fn get(&self, id: Uuid) -> Result<T, crate::error::Error> {
        Err(crate::error::Error {
            message: String::from("not implemented"),
        })
    }
    fn save(&self, obj: &T) -> Result<(), crate::error::Error> {
        Err(crate::error::Error {
            message: String::from("not implemented"),
        })
    }
    fn delete(&self, obj: &T) -> Result<(), crate::error::Error> {
        Err(crate::error::Error {
            message: String::from("not implemented"),
        })
    }
}

#[cfg(test)]
mod tests {

use std::str::FromStr;

use state_handler::StateHandler;

use crate::task::Task;

use super::*;

	#[test]
	fn get_task_from_sqlite() {
        let handler = SqliteHandler{};
        let using_default: Task = handler
            .get(Uuid::from_str("67e55044-10b1-426f-9247-bb680e5fe0c8")
                .unwrap()
            ).unwrap();
        // TODO: convert data to json and compare it after
		let result = Task {
            id: using_default.id,
            description: String::from(""),
            deadline: using_default.time_stamp,
            priority: 5,
            time_stamp: using_default.time_stamp,
        };
		assert_eq!(result, using_default);
	}
}