use super::value::Value;
use colored::*;

pub fn error(msg : String) -> Value {
    println!("ERR: {}", msg.blink().red());
    Value::Nil
}