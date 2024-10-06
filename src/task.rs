use {
    crate::yawt_object::YawtObject,
    chrono::NaiveDateTime, 
    uuid::Uuid,
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
    use {
        super::*, crate::{backend::{ BackendConnector, Backend}, sqlite_backend::SqliteHandler}, std::str::FromStr
    };

    #[test]
    // Test functions create, update and delete.
    fn lifecycle_sqlite() {
        let example = Task {
            id: Uuid::from_str("579c9fce-7919-4851-adf0-f9f4ae1d081b").unwrap(),
            description: String::from_str("Just Do It!").unwrap(),
            deadline: NaiveDateTime::parse_from_str("2022-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            priority: 8,
            time_stamp: NaiveDateTime::parse_from_str("2022-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        };
        let back = SqliteHandler::connect("test.db").unwrap();
        back.save(&example).unwrap();
        let recv_object: Task = back.get(Uuid::from_str("579c9fce-7919-4851-adf0-f9f4ae1d081b").unwrap()).unwrap();
        assert_eq!(example, recv_object);
        back.delete(&recv_object).unwrap();
        let recv_object_after_delete: Result<Task, crate::yawt_object::YawtError> = back.get(Uuid::from_str("579c9fce-7919-4851-adf0-f9f4ae1d081b").unwrap());
        if let Ok(_) = recv_object_after_delete {
            panic!("After delete must return error");
        }
    }
}