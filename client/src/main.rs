extern crate reqwest;
extern crate serde_json;

use serde_json::value::Value;

fn fetch() -> std::result::Result<Value, reqwest::Error> {
    let response: Value = reqwest::get("http://localhost:8000/users/0")?.json()?;
    Ok(response)
}

fn main() {
    println!("Hello, world!");
    let user = fetch().unwrap();
    println!("{}", user);
}
