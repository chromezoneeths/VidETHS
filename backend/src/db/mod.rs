use std::{
	collections::HashMap,
	sync::{Arc, Mutex},
};

use uuid::Uuid;

mod user;
pub use user::User;
mod room;
pub use room::Room;

pub struct Db {
	pub users: HashMap<Uuid, User>,
	pub rooms: HashMap<Uuid, Room>,
}

impl Db {
	pub fn new() -> Arc<Mutex<Db>> {
		Arc::new(Mutex::new(Db {
			users: HashMap::new(),
			rooms: HashMap::new(),
		}))
	}
}
