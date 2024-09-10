use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::{code_span::CodeSpan, location::Location, scanning::Token};

#[derive(Debug)]
pub enum ParsingError {
    UnexpectedEndOfTokenStream(Location),
    UnexpectedToken(Token),
    InvalidAssignmentTarget(CodeSpan),
    TooManyArguments(CodeSpan),
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::UnexpectedEndOfTokenStream(loc) => {
                write!(f, "unexpected end of token stream at {}", loc)
            }
            ParsingError::UnexpectedToken(token) => write!(f, "unexpected token: {}", token),
            ParsingError::InvalidAssignmentTarget(_) => write!(f, "invalid assignment target"),
            ParsingError::TooManyArguments(_) => write!(f, "too many arguments (max 255)"),
        }
    }
}

impl Error for ParsingError {}
