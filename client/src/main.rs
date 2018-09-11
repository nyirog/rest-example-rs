#[macro_use]
extern crate serde_json;

extern crate reqwest;

use serde_json::value::Value;
use std::io::{self, BufRead, Error, ErrorKind};

fn fetch(url: String) -> std::result::Result<Value, reqwest::Error> {
    let response: Value =
        reqwest::get(format!("http://localhost:8000/{}", url).as_str())?.json()?;
    Ok(response)
}

fn do_loop() -> Result<(), Error> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line)?;
    let command: Vec<&str> = line.split_whitespace().collect();

    if command.len() == 0 {
        return Ok(());
    }

    let user = match command[0] {
        "get" => fetch(command[1].to_string()),
        _ => Ok(json!(null)),
    }.map_err(|e| Error::new(ErrorKind::Other, format!("{}", e)))?;

    println!("{}", user);
    do_loop()
}

fn main() -> Result<(), std::io::Error> {
    do_loop()?;
    Ok(())
}
