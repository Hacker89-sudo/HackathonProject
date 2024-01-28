use super::value::Value;

pub fn error(msg : String) -> Value {
    println!("{}", msg);
    Value::Nil
}