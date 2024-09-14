pub mod token_stream;
use std::fmt::{Debug, Display, Formatter};

use crate::code_span::CodeSpan;

#[derive(Debug, PartialEq, Clone)]
/// Represents the type of a token.
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier(String),
    String(String),
    Number(f64),

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Invalid(super::ScanningError),
}

/// Represents a token along with its location in the source code.
#[derive(PartialEq, Clone)]
pub struct Token {
    token: TokenType,
    span: CodeSpan,
}

impl Token {
    /// Creates a new token
    pub fn new(token_type: TokenType, span: CodeSpan) -> Token {
        Token {
            token: token_type,
            span,
        }
    }

    pub fn is_of_type(&self, token_type: TokenType) -> bool {
        self.token == token_type
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self.token, TokenType::Identifier(_))
    }

    pub fn get_type(&self) -> &TokenType {
        &self.token
    }

    pub fn get_span(&self) -> CodeSpan {
        self.span
    }

    pub fn consume(self) -> TokenType {
        self.token
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: '{:?}'", self.span, self.token)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.span, self.token)
    }
}
