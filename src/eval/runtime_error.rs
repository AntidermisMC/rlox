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
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let error_type = match self {
            &RuntimeError::MismatchedTypes(_, _, _) => "Mismatched Type", // TODO better messages
            &RuntimeError::DivisionByZero(span) => "Division by zero",
        };
        write!(f, "{}", error_type)
    }
}

impl Error for RuntimeError {}
