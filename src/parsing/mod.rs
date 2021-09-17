use crate::ast::Expression::*;
use crate::ast::Literal::{False, Nil, NumberLiteral, StringLiteral, True};
use crate::ast::{Binary, BinaryOperator, Expression, Unary, UnaryOperator};
use crate::error::Error;
use crate::scanning::TokenStream;
use crate::scanning::TokenType;
use std::convert::TryFrom;

fn parse_expression(tokens: &mut TokenStream) -> Result<Expression, Error> {
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

fn parse_equality(tokens: &mut TokenStream) -> Result<Expression, Error> {
    let mut expr = try_parse!(parse_comparison, tokens)?;

    while let Some(op) = tokens.peek() {
        if op.is_of_type(TokenType::EqualEqual) || op.is_of_type(TokenType::BangEqual) {
            tokens.next();
            let right = parse_comparison(tokens)?;
            expr = BinaryOperation(Binary {
                operator: BinaryOperator::try_from(&op).unwrap(),
                left: Box::new(expr),
                right: Box::new(right),
            });
        } else {
            break;
        }
    }

    Ok(expr)
}

fn parse_comparison(tokens: &mut TokenStream) -> Result<Expression, Error> {
    let mut expr = try_parse!(parse_term, tokens)?;

    while let Some(op) = tokens.peek() {
        if op.is_of_type(TokenType::Greater)
            || op.is_of_type(TokenType::GreaterEqual)
            || op.is_of_type(TokenType::Less)
            || op.is_of_type(TokenType::LessEqual)
        {
            tokens.next();
            let right = parse_term(tokens)?;
            expr = BinaryOperation(Binary {
                operator: BinaryOperator::try_from(&op)?,
                left: Box::new(expr),
                right: Box::new(right),
            });
        } else {
            break;
        }
    }

    Ok(expr)
}

fn parse_term(tokens: &mut TokenStream) -> Result<Expression, Error> {
    let mut expr = parse_factor(tokens)?;

    while let Some(op) = tokens.peek() {
        if op.is_of_type(TokenType::Plus) || op.is_of_type(TokenType::Minus) {
            tokens.next();
            let right = parse_factor(tokens)?;
            expr = BinaryOperation(Binary {
                operator: BinaryOperator::try_from(&op).unwrap(),
                left: Box::new(expr),
                right: Box::new(right),
            });
        } else {
            break;
        }
    }

    Ok(expr)
}

fn parse_factor(tokens: &mut TokenStream) -> Result<Expression, Error> {
    let mut expr = parse_unary(tokens)?;

    while let Some(op) = tokens.peek() {
        if op.is_of_type(TokenType::Star) || op.is_of_type(TokenType::Slash) {
            tokens.next();
            let right = parse_unary(tokens)?;
            expr = BinaryOperation(Binary {
                operator: BinaryOperator::try_from(&op).unwrap(),
                left: Box::new(expr),
                right: Box::new(right),
            })
        } else {
            break;
        }
    }

    Ok(expr)
}

fn parse_unary(tokens: &mut TokenStream) -> Result<Expression, Error> {
    match tokens.next() {
        None => Err(Error::unexpected_end_of_file(tokens.current_position())),
        Some(tok) => {
            if tok.is_of_type(TokenType::Bang) || tok.is_of_type(TokenType::Minus) {
                let expr = parse_unary(tokens)?;
                Ok(UnaryOperation(Unary {
                    op: UnaryOperator::try_from(&tok).unwrap(),
                    expr: Box::new(expr),
                }))
            } else {
                tokens.back();
                parse_primary(tokens)
            }
        }
    }
}

fn parse_primary(tokens: &mut TokenStream) -> Result<Expression, Error> {
    if let Some(token) = tokens.next() {
        match token.get_type() {
            TokenType::False => Ok(Literal(False)),
            TokenType::True => Ok(Literal(True)),
            TokenType::Nil => Ok(Literal(Nil)),

            TokenType::Number(n) => Ok(Literal(NumberLiteral(*n))),
            TokenType::String(s) => Ok(Literal(StringLiteral(s.to_string()))), // TODO remove cloning

            TokenType::LeftParen => {
                let expr = parse_expression(tokens)?;
                match tokens.next() {
                    Some(t) if t.is_of_type(TokenType::RightParen) => Ok(expr),
                    Some(tok) => Err(Error::new("unexpected token".to_string(), tok.get_span())), // TODO better error handling
                    None => Err(Error::unexpected_end_of_file(tokens.current_position())),
                }
            }

            _ => Err(Error::new("unexpected token".to_string(), token.get_span())), // TODO better error handling
        }
    } else {
        Err(Error::unexpected_end_of_file(tokens.current_position()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_equal_repr(
        to_be_tested: &str,
        parsing_function: fn(&mut TokenStream) -> Result<Expression, Error>,
    ) {
        assert_eq!(
            to_be_tested,
            parsing_function(&mut TokenStream::new(to_be_tested))
                .unwrap()
                .to_string()
        )
    }

    macro_rules! test_cases {
        ($parsing_function:ident, $($case:expr),*) => {
            $(assert_equal_repr($case, $parsing_function);)*
        }
    }

    macro_rules! gen_tests {
        ($name:ident, $function:ident, $($case:expr),*) => {
            #[test]
            fn $name() {
                test_cases!($function, $($case),*);
            }
        };
    }

    gen_tests!(
        primary,
        parse_primary,
        "true",
        "false",
        "nil",
        "\"hi\"",
        "42"
    );

    gen_tests!(
        unary,
        parse_unary,
        "true",
        "!true",
        "!!true",
        "-1",
        "--1",
        "!(1 + 1)"
    );

    gen_tests!(
        factor,
        parse_factor,
        "1",
        "1 * 1",
        "1 / 1",
        "1 * 1 / 1",
        "1 * (1 / 1)",
        "-(1 / 1)"
    );

    gen_tests!(
        term,
        parse_term,
        "1",
        "1 + 1",
        "1 - 1",
        "1 + 1 - 1",
        "1 + (1 - 1)",
        "1 * 1 + 1",
        "1 - 1 * 1"
    );

    gen_tests!(
        comparison,
        parse_comparison,
        "1",
        "1 < 1",
        "1 * 1 > 1",
        "-1 <= 1 + 1",
        "!(1 >= 1)"
    );

    gen_tests!(
        equality,
        parse_equality,
        "1",
        "1 == 1",
        "1 + 1 != 1",
        "true == !false"
    );
}
