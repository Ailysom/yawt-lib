use serde_json::json;

pub trait YawtObject
where Self: serde::Serialize {
	fn new() -> Self;
	fn to_json(&self) -> String {
        json!(self).to_string()
    }
    fn from_sqlite_row(row: &rusqlite::Row) -> Self;// TODO: wrap result to Result
}
/*
|row| {
            // panic!("{:?}",row.get::<usize, String>(4).unwrap());
            Ok(Task {
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
            })
        }
 */