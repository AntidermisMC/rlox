use crate::ast::types::ValueType;
use crate::eval::Result;

pub fn clock(_: Vec<ValueType>) -> Result<ValueType> {
    Ok(ValueType::Number(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Wut")
            .as_secs_f64(),
    ))
}

pub type NativeFunction = fn(Vec<ValueType>) -> Result<ValueType>;

pub fn prelude() -> Vec<(&str, NativeFunction, usize)> {
    vec![("clock", clock, 0)]
}
