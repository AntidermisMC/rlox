use crate::{
    ast::types::{NativeFunction, ValueType},
    code_span::CodeSpan,
    eval::Result,
};

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
            ValueType::String(s) => {
                Ok(ValueType::String(std::rc::Rc::new(format!("Hello, {}", s))))
            }
            _ => Err(crate::eval::runtime_error::RuntimeError::MismatchedTypes(
                span,
                arg.as_type(),
                std::collections::HashSet::from([crate::ast::types::Type::String]),
            )),
        }
    }
    vec![("hello", hello, 1)]
}
