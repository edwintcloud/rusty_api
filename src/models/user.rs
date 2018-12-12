use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use schema::users;
extern crate bcrypt;

#[table_name = "users"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct User {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String
}

impl User {
    pub fn create(mut user: User, connection: &MysqlConnection) -> User {
        use self::bcrypt::{hash};
        match hash(&user.password, 6) {
            Ok(hashed) => user.password = hashed,
            Err(_) => info!("unable to hash")
        };

        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)
            .expect("Error creating new user");

        users::table.order(users::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<User> {
        users::table.order(users::id.asc()).load::<User>(connection).unwrap()
    }

    pub fn update(id: i32, user: User, connection: &MysqlConnection) -> bool {
        diesel::update(users::table.find(id)).set(&user).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(users::table.find(id)).execute(connection).is_ok()
    }
}