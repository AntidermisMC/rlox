use crate::ast::expressions::BinaryOperator;
use crate::ast::expressions::BinaryOperator::*;
use crate::ast::expressions::UnaryOperator;
use crate::ast::expressions::UnaryOperator::*;
use crate::error::Error;
use crate::scanning::{Token, TokenType};
use std::convert::TryFrom;

impl TryFrom<&Token> for BinaryOperator {
    type Error = crate::error::Error;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match *value.get_type() {
            TokenType::BangEqual => Ok(Inequality),
            TokenType::EqualEqual => Ok(Equality),
            TokenType::Greater => Ok(StrictSuperiority),
            TokenType::GreaterEqual => Ok(Superiority),
            TokenType::Less => Ok(StrictInferiority),
            TokenType::LessEqual => Ok(Inferiority),
            TokenType::Plus => Ok(Addition),
            TokenType::Minus => Ok(Subtraction),
            TokenType::Star => Ok(Multiplication),
            TokenType::Slash => Ok(Division),
            _ => Err(Error::new(
                "not a binary operator".to_string(),
                value.get_span(),
            )),
        }
    }
}

impl TryFrom<&Token> for UnaryOperator {
    type Error = crate::error::Error;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match *value.get_type() {
            TokenType::Minus => Ok(Minus),
            TokenType::Bang => Ok(Not),
            _ => Err(Error::new(
                "not an unary operator".to_string(),
                value.get_span(),
            )),
        }
    }
}
