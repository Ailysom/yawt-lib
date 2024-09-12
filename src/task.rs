use chrono::{
    DateTime,
    Utc,
};
use uuid::Uuid;

use crate::YawtObject;

#[derive(Debug,PartialEq)]
pub struct Task {
    pub id: Uuid, // uuid v4
    pub description: String, // Body of task. Format: Markdown
    pub deadline: DateTime<Utc>,
    pub priority: u8, // 1 to 10. Default: 5.
    time_stamp: DateTime<Utc>, // Time of creation,
}

impl YawtObject for Task {}

impl Default for Task {
    fn default() -> Self {
        let now = chrono::offset::Utc::now();
        Task {
            id: Uuid::new_v4(),
            description: String::from(""),
            deadline: now,
            priority: 5,
            time_stamp: now,
        }
    }
}

#[cfg(test)]
mod tests {

use super::*;

	#[test]
	fn default_task() {
        let using_default = Task {
            ..Default::default()
        };
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