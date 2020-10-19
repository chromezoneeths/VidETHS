use super::Db;
use rand::{rngs::OsRng, RngCore};
use serde::Serialize;
use std::{sync::Arc, sync::Mutex, time::Instant};
use uuid::Uuid;

const SECRET_LENGTH: usize = 1024;
const EXPIRE_AFTER: usize = 3600;

#[derive(Clone, PartialEq, Serialize)]
pub struct User {
	pub name: String,
	#[serde(skip_serializing)]
	pub secret: String,
	pub id: Uuid,
	#[serde(skip_serializing)]
	pub last_seen: Instant,
}
impl User {
	pub fn new(name: String) -> User {
		let id = Uuid::new_v4();
		let mut secret = [0u8; SECRET_LENGTH];
		OsRng.fill_bytes(&mut secret);
		let secret = base64::encode(secret);
		User {
			name,
			id,
			secret,
			last_seen: Instant::now(),
		}
	}
	pub fn put(&self, db: Arc<Mutex<Db>>) {
		db.lock().unwrap().users.insert(self.id, self.clone());
	}
}
