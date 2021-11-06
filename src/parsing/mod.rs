mod parsing_error;

#[cfg(test)]
mod tests;

use crate::ast::LiteralValue::{False, Nil, NumberLiteral, StringLiteral, True};
use crate::ast::{Binary, BinaryOperator, Expression, Literal, Unary, UnaryOperator};
use crate::code_span::CodeSpan;
use crate::scanning::TokenType;
use crate::scanning::{Token, TokenStream};
pub use parsing_error::ParsingError;
use std::convert::TryFrom;

type Result<T> = std::result::Result<T, ParsingError>;

pub fn parse(tokens: &mut TokenStream) -> Result<Expression> {
    parse_expression(tokens)
}

fn parse_expression(tokens: &mut TokenStream) -> Result<Expression> {
    parse_equality(tokens)
}

macro_rules! try_parse {
    // TODO maybe remove this ? Not actually necessary atm, maybe when parser is complete
    ($function:ident, $token_stream:ident) => {{
        let save = $token_stream.save_position();
        match $function($token_stream) {
            Ok(expr) => Ok(expr),
            Err(e) => {
                $token_stream.load_position(save);
                Err(e)
            }
        }
    }};
}

fn parse_equality(tokens: &mut TokenStream) -> Result<Expression> {
    let mut expr = try_parse!(parse_comparison, tokens)?;

    while let Some(op) = tokens.peek() {
        if op.is_of_type(TokenType::EqualEqual) || op.is_of_type(TokenType::BangEqual) {
            tokens.next();
            let right = parse_comparison(tokens)?;
            let span = CodeSpan::combine(expr.get_location(), right.get_location());
            expr = Expression::BinaryOperation(Binary {
                operator: BinaryOperator::try_from(&op).unwrap(),
                left: Box::new(expr),
                right: Box::new(right),
                location: span,
            });
        } else {
            break;
        }
    }

    Ok(expr)
}

fn parse_comparison(tokens: &mut TokenStream) -> Result<Expression> {
    let mut expr = try_parse!(parse_term, tokens)?;

    while let Some(op) = tokens.peek() {
        if op.is_of_type(TokenType::Greater)
            || op.is_of_type(TokenType::GreaterEqual)
            || op.is_of_type(TokenType::Less)
            || op.is_of_type(TokenType::LessEqual)
        {
            tokens.next();
            let right = parse_term(tokens)?;
            let span = CodeSpan::combine(expr.get_location(), right.get_location());
            expr = Expression::BinaryOperation(Binary {
                operator: BinaryOperator::try_from(&op).unwrap(),
                left: Box::new(expr),
                right: Box::new(right),
                location: span,
            });
        } else {
            break;
        }
    }

    Ok(expr)
}

fn parse_term(tokens: &mut TokenStream) -> Result<Expression> {
    let mut expr = parse_factor(tokens)?;

    while let Some(op) = tokens.peek() {
        if op.is_of_type(TokenType::Plus) || op.is_of_type(TokenType::Minus) {
            tokens.next();
            let right = parse_factor(tokens)?;
            let span = CodeSpan::combine(expr.get_location(), right.get_location());
            expr = Expression::BinaryOperation(Binary {
                operator: BinaryOperator::try_from(&op).unwrap(),
                left: Box::new(expr),
                right: Box::new(right),
                location: span,
            });
        } else {
            break;
        }
    }

    Ok(expr)
}

fn parse_factor(tokens: &mut TokenStream) -> Result<Expression> {
    let mut expr = parse_unary(tokens)?;

    while let Some(op) = tokens.peek() {
        if op.is_of_type(TokenType::Star) || op.is_of_type(TokenType::Slash) {
            tokens.next();
            let right = parse_unary(tokens)?;
            let span = CodeSpan::combine(expr.get_location(), right.get_location());
            expr = Expression::BinaryOperation(Binary {
                operator: BinaryOperator::try_from(&op).unwrap(),
                left: Box::new(expr),
                right: Box::new(right),
                location: span,
            })
        } else {
            break;
        }
    }

    Ok(expr)
}

fn parse_unary(tokens: &mut TokenStream) -> Result<Expression> {
    match tokens.next() {
        None => Err(ParsingError::UnexpectedEndOfTokenStream(
            tokens.current_position(),
        )),
        Some(tok) => {
            if tok.is_of_type(TokenType::Bang) || tok.is_of_type(TokenType::Minus) {
                let expr = parse_unary(tokens)?;
                Ok(Expression::UnaryOperation(Unary {
                    op: UnaryOperator::try_from(&tok).unwrap(),
                    expr: Box::new(expr),
                    location: tok.get_span(),
                }))
            } else {
                tokens.back();
                parse_primary(tokens)
            }
        }
    }
}

fn parse_primary(tokens: &mut TokenStream) -> Result<Expression> {
    if let Some(token) = tokens.next() {
        let span = token.get_span();
        match token.consume() {
            TokenType::False => Ok(Expression::Literal(Literal::new(False, span))),
            TokenType::True => Ok(Expression::Literal(Literal::new(True, span))),
            TokenType::Nil => Ok(Expression::Literal(Literal::new(Nil, span))),

            TokenType::Number(n) => Ok(Expression::Literal(Literal::new(NumberLiteral(n), span))),
            TokenType::String(s) => Ok(Expression::Literal(Literal::new(StringLiteral(s), span))),

            TokenType::LeftParen => {
                let expr = parse_expression(tokens)?;
                match tokens.next() {
                    Some(t) if t.is_of_type(TokenType::RightParen) => Ok(expr),
                    Some(tok) => Err(ParsingError::UnexpectedToken(tok)),
                    None => Err(ParsingError::UnexpectedEndOfTokenStream(
                        tokens.current_position(),
                    )),
                }
            }

            invalid_token => Err(ParsingError::UnexpectedToken(Token::new(
                invalid_token,
                span,
            ))),
        }
    } else {
        Err(ParsingError::UnexpectedEndOfTokenStream(
            tokens.current_position(),
        ))
    }
}
