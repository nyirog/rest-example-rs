extern crate serde_json;

extern crate reqwest;

use serde_json::value::Value;
use std::io::{self, BufRead, Error as IoError, ErrorKind};

enum Command {
    Get(String),
    List(),
    Invalid(String),
}

fn parse(words: Vec<String>) -> Command {
    match words[0].as_str() {
        "get" => Command::Get(words[1].to_string()),
        "list" => Command::List(),
        _ => Command::Invalid(words[0].to_string()),
    }
}

fn execute(command: Command, stack: &mut Vec<Value>) -> Result<(), IoError> {
    match command {
        Command::Get(url) => fetch(stack, url),
        Command::List() => list_stack(stack.to_vec()),
        Command::Invalid(command) => Err(IoError::new(
            ErrorKind::Other,
            format!("Invalid command {}", command),
        )),
    }
}

fn list_stack(stack: Vec<Value>) -> Result<(), IoError> {
    let v: Vec<String> = stack.iter().map(|v| v.to_string()).collect();
    println!("{}", v.join("\n"));
    Ok(())
}

fn fetch(stack: &mut Vec<Value>, url: String) -> Result<(), IoError> {
    let mut response =
        reqwest::get(format!("http://localhost:8000/{}", url).as_str()).map_err(from)?;
    let value: Value = response.json().map_err(from)?;
    println!("{}", value);
    stack.push(value);
    Ok(())
}

fn from(error: reqwest::Error) -> IoError {
    IoError::new(ErrorKind::Other, format!("{}", error))
}

fn do_loop(mut stack: Vec<Value>) -> Result<(), IoError> {
    let mut line: String = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line)?;

    let words: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    if words.len() == 0 {
        return Ok(());
    }

    match execute(parse(words), &mut stack) {
        Err(e) => println!("{}", e),
        Ok(_) => (),
    };

    do_loop(stack)
}

fn main() {
    let stack: Vec<Value> = vec![];
    match do_loop(stack) {
        Ok(r) => r,
        Err(e) => println!("Error: {}", e),
    }
}
