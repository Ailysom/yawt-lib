use {
    crate::yawt_object::YawtObject, chrono::{
        DateTime, NaiveDateTime, Utc
    }, std::str::FromStr, uuid::Uuid
};


#[derive(Debug,PartialEq,serde::Serialize)]
pub struct Task {
    pub id: Uuid, // uuid v4
    pub description: String, // Body of task. Format: Markdown
    pub deadline: NaiveDateTime,
    pub priority: u8, // 1 to 10. Default: 5.
    pub time_stamp: NaiveDateTime, // Time of creation,
}


impl YawtObject for Task {
    fn new() -> Self {
        return Task {..Default::default()};
    }
    fn from_sqlite_row(row: &rusqlite::Row) -> Self {
        Task {
            id: Uuid::from_str(&(row.get::<usize, String>(0).unwrap())).unwrap(),
            description: row.get(1).unwrap(),
            deadline: NaiveDateTime::parse_from_str(
                &(row.get::<usize, String>(5).unwrap()),
                "%Y-%m-%d %H:%M:%S",
            ).unwrap(),
            time_stamp: NaiveDateTime::parse_from_str(
                &(row.get::<usize, String>(5).unwrap()),
                "%Y-%m-%d %H:%M:%S",
            ).unwrap(),
            priority: row.get::<usize, u8>(2).unwrap(),
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        let now = chrono::offset::Utc::now().naive_local();
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
        let now = using_default.time_stamp;
        let result = format!(
            "{{\"deadline\":\"{}\",\"description\":\"\",\"id\":\"{:?}\",\"priority\":5,\"time_stamp\":\"{}\"}}",
            now,
            using_default.id,
            now,
        );
		assert_eq!(result, using_default.to_json());
	}
}