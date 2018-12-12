#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate log;

use std::collections::HashMap;
use rocket::config::{Config, Environment, Value};
use rocket_contrib::json::{Json, JsonValue};

mod schema;
mod models;
mod controllers;

// Global database struct
#[database("users")]
pub struct UsersDbConn(diesel::MysqlConnection);

// Index route
#[get("/")]
fn index() -> &'static str {
    "Hello rusty world 🎉"
}

fn main() {

    // bring users controller into scope from controllers module
    use controllers::users;

    // startup rocket application with / root context
    // Ignite uses Rocket.toml for config
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api/v1/users", routes![users::create, users::read, users::update, users::delete])
        .attach(UsersDbConn::fairing())
        .launch();
}