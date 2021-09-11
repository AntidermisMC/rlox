use crate::code_span::CodeSpan;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq)]
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

    Invalid(crate::error::Error),
}

/// Represents a token along with its location in the source code.
#[derive(PartialEq)]
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
