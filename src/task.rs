use chrono::{
    DateTime,
    Utc,
};
use serde_json::json;
use uuid::Uuid;

use crate::YawtObject;

#[derive(Debug,PartialEq,serde::Serialize)]
pub struct Task {
    pub id: Uuid, // uuid v4
    pub description: String, // Body of task. Format: Markdown
    pub deadline: DateTime<Utc>,
    pub priority: u8, // 1 to 10. Default: 5.
    pub time_stamp: DateTime<Utc>, // Time of creation,
}

impl YawtObject for Task {
    fn new() -> Self {
        return Task {..Default::default()};
    }
    fn to_json(&self) -> String {
        json!(self).to_string()
    }
}

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

use std::str::FromStr;

use super::*;

	#[test]
	fn default_task() {
        let using_default = Task {
            ..Default::default()
        };
        // TODO: convert data to json and compare it after
		let result = String::from_str("{\"deadline\":\"2024-09-12T23:55:29.060642061Z\",\"description\":\"\",\"id\":\"b0968c56-11ed-445f-b4e2-db082e30149f\",\"priority\":5,\"time_stamp\":\"2024-09-12T23:55:29.060642061Z\"}").unwrap();
		assert_eq!(result, using_default.to_json());
	}
}