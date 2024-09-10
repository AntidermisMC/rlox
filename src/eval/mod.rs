pub use builtins::prelude;
use runtime_error::RuntimeError;

use crate::{
    ast::types::{NativeFunction, Type, Value, ValueType},
    eval::{environment::Environment, output_stream::OutputStream},
};

mod builtins;
mod environment;
mod expressions;
pub mod output_stream;
mod runtime_error;
mod statements;

#[cfg(test)]
mod tests;

pub struct Evaluator {
    env: Environment,
    out: OutputStream,
}

impl Evaluator {
    pub fn new(out: OutputStream) -> Self {
        Evaluator {
            env: Environment::new(),
            out,
        }
    }

    pub fn register_prelude(&mut self, prelude: Vec<(&str, NativeFunction, usize)>) {
        for (name, function, arity) in prelude {
            self.env
                .define(name.to_string(), ValueType::NativeFunction(function, arity));
        }
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

fn is_truthy(value: &ValueType) -> bool {
    match value {
        ValueType::Boolean(false) | ValueType::Nil => false,
        _ => true,
    }
}
