#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

mod api;
mod db;
mod stats;
use db::Db;
use rocket_contrib::serve::StaticFiles;
use stats::Stats;

fn main() {
	// Init db
	let db = Db::new();
	// Init stats
	// let stats = Stats::new();
	// Init rocket
	rocket::ignite()
		.manage(db)
		// .manage(stats)
		.mount("/", StaticFiles::from("./static"))
		.mount("/api", routes![api::v1::ping]);
}
