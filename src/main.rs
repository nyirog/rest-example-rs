#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use rocket_contrib::Json;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
}

lazy_static! {
    static ref USERS: Mutex<HashMap<u32, User>> = {
        let mut m = HashMap::new();
        m.insert(
            0,
            User {
                name: "admin".to_string(),
            },
        );
        Mutex::new(m)
    };
}

#[get("/")]
fn list() -> Json<HashMap<u32, User>> {
    let users = USERS.lock().unwrap();
    Json((*users).clone())
}

#[get("/<user_id>")]
fn get(user_id: u32) -> Json<User> {
    let users = USERS.lock().unwrap();
    Json((*users).get(&user_id).unwrap().clone())
}

fn main() {
    rocket::ignite().mount("/users", routes![list, get]).launch();
}
