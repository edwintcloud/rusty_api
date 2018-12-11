use rocket_contrib::json::{Json, JsonValue};
use models::user::{User};
use {UsersDbConn};

// CREATE
#[post("/", data = "<user>")]
pub fn create(user: Json<User>, conn: UsersDbConn ) -> Json<User> {
    let insert = User { id: None, ..user.into_inner() };
    Json(User::create(insert, &conn))
}

// READ
#[get("/")]
pub fn read(conn: UsersDbConn) -> Json<JsonValue> {
    Json(json!(User::read(&conn)))
}

// UPDATE
#[put("/<_id>", data = "<user>")]
pub fn update(_id: i32, user: Json<User>, conn: UsersDbConn) -> Json<JsonValue> {
    let update = User { id: Some(_id), ..user.into_inner() };
    Json(json!({
        "success": User::update(_id, update, &conn)
    }))
}

// DELETE
#[delete("/<_id>")]
pub fn delete(_id: i32, conn: UsersDbConn) -> Json<JsonValue> {
    Json(json!({
        "success": User::delete(_id, &conn)
    }))
}