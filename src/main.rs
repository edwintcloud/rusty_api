#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use std::collections::HashMap;
use rocket::config::{Config, Environment, Value};
use rocket_contrib::json::{Json, JsonValue};

mod schema;
mod user;
use user::{User};

// Database
#[database("users")]
struct UsersDbConn(diesel::MysqlConnection);

// Index route
#[get("/")]
fn index() -> &'static str {
    "Hello rusty world 🎉"
}

// CREATE
#[post("/", data = "<user>")]
fn create(user: Json<User>, conn: UsersDbConn ) -> Json<User> {
    let insert = User { id: None, ..user.into_inner() };
    Json(User::create(insert, &conn))
}

// READ
#[get("/")]
fn read(conn: UsersDbConn) -> Json<JsonValue> {
    Json(json!(User::read(&conn)))
}

// UPDATE
#[put("/<_id>", data = "<user>")]
fn update(_id: i32, user: Json<User>) -> Json<User> {
    user
}

// DELETE
#[delete("/<_id>")]
fn delete(_id: i32) -> Json<JsonValue> {
    Json(json!({
        "message": "User deleted successfully"
    }))
}

fn main() {
    // startup rocket application with / root context
    // Ignite uses Rocket.toml for config
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/api/v1/users", routes![create, read, update, delete])
        .attach(UsersDbConn::fairing())
        .launch();
}