mod task;
mod state_handler;
mod error;
mod sqlite_handler;
mod schema;

trait YawtObject {
	fn new() -> Self;
	fn to_json(&self) -> String;
}

pub fn add(left: u64, right: u64) -> u64 {
	left + right
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}
