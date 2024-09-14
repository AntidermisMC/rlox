use std::{
    collections::HashSet,
    error::Error,
    fmt::{Display, Formatter},
};

use crate::{ast::types::Value, code_span::CodeSpan, eval::Type};

#[derive(Debug)]
pub enum RuntimeError {
    /// Location: the location of the errored value.
    /// Type: the actual type of the value.
    /// HashSet<Type>: the allowed types for the value.
    /// InvalidArgumentCount(span, expected count, actual count)
    MismatchedTypes(CodeSpan, Type, HashSet<Type>),
    DivisionByZero(CodeSpan),
    UnboundName(CodeSpan, String),
    WriteError(CodeSpan),
    NotCallable(CodeSpan),
    InvalidArgumentCount(CodeSpan, usize, usize),
    Return(Value),
}

impl RuntimeError {
    pub fn location(&self) -> &CodeSpan {
        match self {
            RuntimeError::MismatchedTypes(span, _, _) => span,
            RuntimeError::DivisionByZero(span) => span,
            RuntimeError::UnboundName(span, _) => span,
            RuntimeError::WriteError(span) => span,
            RuntimeError::NotCallable(span) => span,
            RuntimeError::InvalidArgumentCount(span, _, _) => span,
            RuntimeError::Return(value) => &value.location,
        }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let error_type = match self {
            RuntimeError::MismatchedTypes(_, _, _) => "Mismatched Type".to_string(),
            RuntimeError::DivisionByZero(_) => "Division by zero".to_string(),
            RuntimeError::UnboundName(_, ident) => format!("Unbound name {}", ident),
            RuntimeError::WriteError(_) => "Write failed".to_string(),
            RuntimeError::NotCallable(_) => "Not a callable object".to_string(),
            RuntimeError::InvalidArgumentCount(_, expected, actual) => format!(
                "Invalid argument count (expected {}, got {}",
                expected, actual
            ),
            RuntimeError::Return(_) => "Return outside function".to_string(),
        };
        write!(f, "{}: {}", self.location(), error_type)
    }
}

impl Error for RuntimeError {}
