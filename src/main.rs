#![feature(plugin, custom_derive, const_fn)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;

mod static_files;
mod db;
mod robot;
mod routes;
mod schema;

use dotenv::dotenv;
use std::env;

use rocket::Rocket;
use routes::*;

/// revisit rocket/examples/json
fn rocket() -> Rocket {
    dotenv().ok();

    // Url to the database as set in the DATABASE_URL environment variable.
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Initializes database pool with r2d2.
    let pool = db::init_pool(database_url);

    rocket::ignite()
        .manage(pool)
        .mount("/app", routes![static_files::index, static_files::all])
        .mount("/api/robots", routes![index, new, show, delete, department])
}

fn main() {
    rocket().launch();
}
