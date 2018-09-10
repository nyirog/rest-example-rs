extern crate reqwest;
extern crate serde_json;

use serde_json::value::Value;
use std::io::{self, BufRead};


fn fetch(url: String) -> std::result::Result<Value, reqwest::Error> {
    let response: Value = reqwest::get(format!("http://localhost:8000/{}", url).as_str())?.json()?;
    Ok(response)
}

fn do_loop() -> Result<(), std::io::Error>{
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line)?;

    if line.len() == 0 {
        return Ok(())
    }

    let user = fetch(line).unwrap();
    println!("{}", user);
    do_loop()
}

fn main() -> Result<(), std::io::Error>{
    do_loop()?;
    Ok(())
}
