extern crate serde_json;

extern crate reqwest;

use serde_json::value::Value;
use std::io::{self, BufRead, Error as IoError, ErrorKind};

fn fetch(url: String) -> std::result::Result<Value, reqwest::Error> {
    let response: Value =
        reqwest::get(format!("http://localhost:8000/{}", url).as_str())?.json()?;
    Ok(response)
}

fn do_loop() -> Result<(), IoError> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line)?;
    let command: Vec<&str> = line.split_whitespace().collect();

    if command.len() == 0 {
        return Ok(());
    }

    let user = match command[0] {
        "get" => fetch(command[1].to_string())
            .map_err(|e| IoError::new(ErrorKind::Other, format!("{}", e))),
        _ => Err(IoError::new(
            ErrorKind::Other,
            format!("Invalid command: {}", command[0]),
        )),
    }?;

    println!("{}", user);
    do_loop()
}

fn main() {
    match do_loop() {
        Ok(r) => r,
        Err(e) => println!("Error: {}", e),
    }
}
