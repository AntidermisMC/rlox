use crate::code_span::CodeSpan;
use std::error::Error;
use std::fmt::{Display, Formatter};

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
