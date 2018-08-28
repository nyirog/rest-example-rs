#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

use rocket::http::Status;
use rocket::response::status;
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
fn list(common_users: State<Mutex<Users>>) -> Result<Json<Users>, status::Custom<String>> {
    let users = (*common_users.inner()).lock().map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            format!("Resource is locked by other user\n"),
        )
    })?;
    Ok(Json(users.clone()))
}

#[get("/<user_id>")]
fn get(
    common_users: State<Mutex<Users>>,
    user_id: u32,
) -> Result<Json<User>, status::Custom<String>> {
    let users = (*common_users.inner()).lock().map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            format!("Resource is locked by other user\n"),
        )
    })?;
    let user = users.get(&user_id).ok_or(status::Custom(
        Status::NotFound,
        format!("Invalid user id: {}\n", user_id),
    ))?;
    Ok(Json(user.clone()))
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
