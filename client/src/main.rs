extern crate serde_json;

extern crate reqwest;

use serde_json::value::Value;
use std::io::{self, BufRead, Error as IoError, ErrorKind};

enum Command {
    Get(String),
    Post(String),
    List(),
    Pop(),
    View(String),
    Set(String, String),
    Help(),
    Invalid(String),
}

fn parse(words: Vec<String>) -> Command {
    match words.len() {
        0 => Command::Invalid("".to_string()),
        1 => match words[0].as_str() {
            "list" => Command::List(),
            "pop" => Command::Pop(),
            "help" => Command::Help(),
            _ => Command::Invalid(words[0].to_string()),
        },
        2 => match words[0].as_str() {
            "get" => Command::Get(words[1].to_string()),
            "post" => Command::Post(words[1].to_string()),
            "view" => Command::View(words[1].to_string()),
            _ => Command::Invalid(words[0].to_string()),
        },
        3 => match words[0].as_str() {
            "set" => Command::Set(words[1].to_string(), words[2].to_string()),
            _ => Command::Invalid(words[0].to_string()),
        },
        _ => Command::Invalid(words[0].to_string()),
    }
}

fn execute(command: Command, stack: &mut Vec<Value>) -> Result<(), IoError> {
    match command {
        Command::Get(url) => fetch(stack, url),
        Command::Post(url) => post(stack.to_vec(), url),
        Command::List() => list_stack(stack.to_vec()),
        Command::Pop() => pop_stack(stack),
        Command::View(key) => view_stack_item(stack, key),
        Command::Set(key, value) => set_stack_item(stack, key, value),
        Command::Help() => help(),
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
    println!("Response status: {}", response.status());
    let value: Value = response.json().map_err(from)?;
    println!("{}", value);
    stack.insert(0, value);
    Ok(())
}

fn post(stack: Vec<Value>, url: String) -> Result<(), IoError> {
    if stack.len() == 0 {
        Err(IoError::new(ErrorKind::Other, "Stack is empty"))
    } else {
        let value = stack[0].clone();
        let client = reqwest::Client::new();
        let response = client
            .post(format!("http://localhost:8000/{}", url).as_str())
            .json(&value)
            .send()
            .map_err(from)?;
        println!("Response status: {}", response.status());
        Ok(())
    }
}

fn from(error: reqwest::Error) -> IoError {
    IoError::new(ErrorKind::Other, format!("{}", error))
}

fn pop_stack(stack: &mut Vec<Value>) -> Result<(), IoError> {
    let value: Value = stack
        .pop()
        .ok_or(IoError::new(ErrorKind::Other, "Stack is empty"))?;
    println!("{}", value);
    Ok(())
}

fn view_stack_item(stack: &mut Vec<Value>, key: String) -> Result<(), IoError> {
    if stack.len() == 0 {
        Err(IoError::new(ErrorKind::Other, "Stack is empty"))
    } else {
        let value = stack[0].clone();
        let item: Value = value
            .get(key)
            .ok_or(IoError::new(ErrorKind::Other, format!("Invalid key")))?
            .clone();
        println!("{}", item);
        stack.insert(0, item);
        Ok(())
    }
}

fn set_stack_item(stack: &mut Vec<Value>, key: String, value: String) -> Result<(), IoError> {
    if stack.len() == 0 {
        Err(IoError::new(ErrorKind::Other, "Stack is empty"))
    } else {
        let mut stack_value = stack[0].clone();
        let json_value: Value = serde_json::from_str(value.as_str())?;
        if stack_value.is_object() {
            stack_value[key] = json_value;
            println!("{}", stack_value);
            stack.insert(0, stack_value);
            Ok(())
        } else {
            Err(IoError::new(
                ErrorKind::Other,
                "Stack head is not an object",
            ))
        }
    }
}

fn help() -> Result<(), IoError> {
    println!(
        "rest-client
    Interactive REST client which fetches json data from http://localhost:8000
    and stores it in a stack for further manipulation (view, set) and to post it back.
    head refers to the first element of the stack which can be manipulated or posted

get <url>
    fetch the url and save the response to the head of the stack
post <url>
    post the head of the stack to the url
list
    print the stack
pop
    remove the head of the stack
view <key>
    create a new stack element from the value of the key of the stack-head
set <key> <json-data>
    create a new stack-head by changing the value of the current head under the given key
"
    );
    Ok(())
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
