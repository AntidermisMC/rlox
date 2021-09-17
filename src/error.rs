use crate::code_span::CodeSpan;
use crate::location::Location;
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

    pub fn unexpected_end_of_file(position: Location) -> Self {
        Error::new(
            "Unexpected EOF".to_string(),
            CodeSpan::new(position, position),
        )
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
