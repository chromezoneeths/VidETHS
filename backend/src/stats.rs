use std::{
	collections::{HashMap, VecDeque},
	sync::Arc,
	sync::Mutex,
	time::Instant,
};

pub struct Stats {
	pub records: Vec<(String, u128)>,
}
impl Stats {
	pub fn new() -> Arc<Mutex<Stats>> {
		Arc::new(Mutex::new(Stats { records: vec![] }))
	}
	pub fn start(category: String) -> (String, Instant) {
		(category, Instant::now())
	}
	pub fn finish(&mut self, timer: (String, Instant)) {
		let vec = self.records.push((timer.0, timer.1.elapsed().as_millis()));
	}
}
#[macro_export]
macro_rules! timed {
	(category: String, stats: Arc<Mutex<Stats>>, $timer: ident, $closure: tt) => {
		let $timer = stats.start();
		$closure();
		stats.finish($timer);
	};
}
