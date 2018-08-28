#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

use rocket::State;
use rocket_contrib::Json;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
}

type Users = HashMap<u32, User>;

#[get("/")]
fn list(users: State<Mutex<Users>>) -> Json<HashMap<u32, User>> {
    Json((*users.inner()).lock().unwrap().clone())
}

#[get("/<user_id>")]
fn get(users: State<Mutex<Users>>, user_id: u32) -> Json<User> {
    Json(
        (*users.inner())
            .lock()
            .unwrap()
            .get(&user_id)
            .unwrap()
            .clone(),
    )
}

fn init_users() -> Mutex<Users> {
    let mut m = HashMap::new();
    m.insert(
        0,
        User {
            name: "admin".to_string(),
        },
    );
    Mutex::new(m)
}

fn main() {
    rocket::ignite()
        .mount("/users", routes![list, get])
        .manage(init_users())
        .launch();
}
