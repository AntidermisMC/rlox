use crate::code_span::CodeSpan;
use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq, Clone)]
pub struct Error {
    message: String,
    location: CodeSpan,
}

impl Error {
    pub fn new(message: String, location: CodeSpan) -> Self {
        Error { message, location }
    }

}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} error: {}", self.location, self.message)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <&Self as Display>::fmt(&self, f)
    }
}

impl std::error::Error for Error {}
