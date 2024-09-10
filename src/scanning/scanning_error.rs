use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::code_span::CodeSpan;

#[derive(Debug, PartialEq, Clone)]
pub enum ScanningError {
    UnterminatedString(CodeSpan),
    InvalidCharacter(char, CodeSpan),
}

impl Display for ScanningError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            ScanningError::UnterminatedString(span) => write!(f, "unterminated string at {}", span),
            ScanningError::InvalidCharacter(c, span) => {
                write!(f, "invalid character '{}' at {}", c, span)
            }
        }
    }
}

impl Error for ScanningError {}
