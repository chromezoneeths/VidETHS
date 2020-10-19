use super::Db;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone, PartialEq, Serialize)]
pub struct Room {
	pub name: String,
	pub owners: Vec<Uuid>,
	pub members: Vec<(Uuid, bool)>,
	pub id: Uuid,
}
impl Room {
	pub fn new(name: String, creator: Uuid) -> Room {
		let id = Uuid::new_v4();
		Room {
			name,
			owners: vec![creator],
			members: vec![(creator, false)],
			id,
		}
	}
	pub fn put(&self, db: Arc<Mutex<Db>>) {
		db.lock().unwrap().rooms.insert(self.id, self.clone());
	}
}
