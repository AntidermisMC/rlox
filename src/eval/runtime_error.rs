use crate::code_span::CodeSpan;
use crate::eval::Type;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum RuntimeError {
    /// Location: the location of the errored value.
    /// Type: the actual type of the value.
    /// HashSet<Type>: the allowed types for the value.
    MismatchedTypes(CodeSpan, Type, HashSet<Type>),
    DivisionByZero(CodeSpan),
    UnboundName(CodeSpan, String),
}

impl RuntimeError {
    pub fn location(&self) -> &CodeSpan {
        match self {
            RuntimeError::MismatchedTypes(span, _, _) => span,
            RuntimeError::DivisionByZero(span) => span,
            RuntimeError::UnboundName(span, _) => span,
        }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let error_type = match &self {
            &RuntimeError::MismatchedTypes(_, _, _) => "Mismatched Type".to_string(),
            &RuntimeError::DivisionByZero(_) => "Division by zero".to_string(),
            &RuntimeError::UnboundName(_, ident) => format!("Unbound name {}", ident),
        };
        write!(f, "{}: {}", self.location(), error_type)
    }
}

impl Error for RuntimeError {}
