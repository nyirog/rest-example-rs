extern crate serde_json;

extern crate reqwest;

use serde_json::value::Value;
use std::io::{self, BufRead, Error as IoError, ErrorKind};

fn fetch(url: String) -> std::result::Result<Value, reqwest::Error> {
    let response: Value =
        reqwest::get(format!("http://localhost:8000/{}", url).as_str())?.json()?;
    Ok(response)
}

fn execute(mut stack: Vec<Value>, url: String) -> Result<Vec<Value>, IoError> {
    let value = fetch(url).map_err(|e| IoError::new(ErrorKind::Other, format!("{}", e)))?;
    println!("{}", value);
    stack.push(value);
    Ok(stack)
}

fn do_loop(stack: Vec<Value>) -> Result<(), IoError> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line)?;
    let command: Vec<&str> = line.split_whitespace().collect();

    if command.len() == 0 {
        return Ok(());
    }

    let new_stack = match command[0] {
        "get" => execute(stack, command[1].to_string()),
        _ => Err(IoError::new(
            ErrorKind::Other,
            format!("Invalid command: {}", command[0]),
        )),
    }?;

    do_loop(new_stack)
}

fn main() {
    let stack: Vec<Value> = vec![];
    match do_loop(stack) {
        Ok(r) => r,
        Err(e) => println!("Error: {}", e),
    }
}
