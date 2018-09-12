extern crate serde_json;

extern crate reqwest;

use serde_json::value::Value;
use std::io::{self, BufRead, Error as IoError, ErrorKind};

struct Get {
    url: String,
}

impl Get {
    fn new(url: String) -> Self {
        Get { url: url }
    }
}

trait Command {
    fn execute(&self, stack: &mut Vec<Value>) -> Result<(), IoError>;
}

impl Command for Get {
    fn execute(&self, stack: &mut Vec<Value>) -> Result<(), IoError> {
        let mut response =
            reqwest::get(format!("http://localhost:8000/{}", self.url).as_str()).map_err(from)?;
        let value: Value = response.json().map_err(from)?;
        println!("{}", value);
        stack.push(value);
        Ok(())
    }
}

fn from(error: reqwest::Error) -> IoError {
    IoError::new(ErrorKind::Other, format!("{}", error))
}

fn parse(words: Vec<String>) -> Result<impl Command, IoError> {
    match words[0].as_str() {
        "get" => Ok(Get::new(words[1].to_string())),
        _ => Err(IoError::new(
            ErrorKind::Other,
            format!("Invalid command: {}", words[0]),
        )),
    }
}

fn do_loop(mut stack: Vec<Value>) -> Result<(), IoError> {
    let mut line: String = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line)?;

    let words: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    if words.len() == 0 {
        return Ok(());
    }

    let command = parse(words)?;
    match command.execute(&mut stack) {
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
