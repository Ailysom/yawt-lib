use std::str::FromStr;

use chrono::{
    DateTime,
    Utc,
};
use serde_json::json;
use diesel::{backend::Backend, prelude::*, deserialize};

use uuid::Uuid;
use crate::YawtObject;

#[derive(Debug,PartialEq,serde::Serialize,Queryable,Selectable)]
#[diesel(table_name = crate::schema::task)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: Uuid, // uuid v4
    pub description: String, // Body of task. Format: Markdown
    pub deadline: DateTime<Utc>,
    pub priority: u8, // 1 to 10. Default: 5.
    pub time_stamp: DateTime<Utc>, // Time of creation,
}

struct YawtUuid(uuid::Uuid);

impl<DB> Queryable<Text, DB> for LowercaseString
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    type Row = String;

    fn build(s: String) -> deserialize::Result<Self> {
        Ok(LowercaseString(s.to_lowercase()))
    }
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

use super::*;

	#[test]
	fn default_task() {
        let using_default = Task {
            ..Default::default()
        };
        // because by default deadline eq timestamp & it's public
        let now = using_default.deadline.to_rfc3339_opts(
            chrono::SecondsFormat::Nanos,
            true
        );
        let result = format!(
            "{{\"deadline\":\"{}\",\"description\":\"\",\"id\":\"{:?}\",\"priority\":5,\"time_stamp\":\"{}\"}}",
            now,
            using_default.id,
            now,
        );
		assert_eq!(result, using_default.to_json());
	}
}