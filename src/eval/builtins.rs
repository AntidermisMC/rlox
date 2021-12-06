use crate::ast::types::{NativeFunction, Type, ValueType};
use crate::code_span::CodeSpan;
use crate::eval::runtime_error::RuntimeError;
use crate::eval::Result;
use std::collections::HashSet;
use std::rc::Rc;

fn clock(_: Vec<ValueType>, _: CodeSpan) -> Result<ValueType> {
    Ok(ValueType::Number(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Wut")
            .as_secs_f64(),
    ))
}

pub fn prelude() -> Vec<(&'static str, NativeFunction, usize)> {
    vec![("clock", clock, 0)]
}

#[cfg(test)]
pub fn test_prelude() -> Vec<(&'static str, NativeFunction, usize)> {
    fn hello(args: Vec<ValueType>, span: CodeSpan) -> Result<ValueType> {
        let arg = args
            .first()
            .expect("native function called with incorrect number of arguments");
        match arg {
            ValueType::String(s) => Ok(ValueType::String(Rc::new(format!("Hello, {}", s)))),
            _ => Err(RuntimeError::MismatchedTypes(
                span,
                arg.as_type(),
                HashSet::from([Type::String]),
            )),
        }
    }
    vec![("hello", hello, 1)]
}
