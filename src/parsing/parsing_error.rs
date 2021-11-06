use crate::location::Location;
use crate::scanning::Token;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParsingError {
    UnexpectedEndOfTokenStream(Location),
    UnexpectedToken(Token),
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::UnexpectedEndOfTokenStream(loc) => {
                write!(f, "unexpected end of token stream at {}", loc)
            }
            ParsingError::UnexpectedToken(token) => write!(f, "unexpected token: {}", token),
        }
    }
}

impl Error for ParsingError {}
