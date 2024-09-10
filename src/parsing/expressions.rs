use std::convert::TryFrom;

use crate::{
    ast::{
        expressions::{
            Assignment, Binary, BinaryOperator, Call, Expression, Identifier, Literal, Unary,
            UnaryOperator,
        },
        LiteralValue::{False, Nil, NumberLiteral, StringLiteral, True},
    },
    code_span::CodeSpan,
    parsing::{consume, try_parse, ParsingError, Result},
    scanning::{Token, TokenStream, TokenType},
};

pub fn parse_expression(tokens: &mut TokenStream) -> Result<Expression> {
    parse_assignment(tokens)
}

fn parse_logic_or(tokens: &mut TokenStream) -> Result<Expression> {
    let mut expr = parse_logic_and(tokens)?;

    while let Some(token) = tokens.peek() {
        if token.is_of_type(TokenType::Or) {
            tokens.next();
            let right = parse_logic_and(tokens)?;
            let span = CodeSpan::new(expr.get_location().start, right.get_location().end);
            expr = Expression::BinaryOperation(Binary {
                operator: BinaryOperator::Disjunction,
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

fn parse_logic_and(tokens: &mut TokenStream) -> Result<Expression> {
    let mut expr = parse_equality(tokens)?;

    while let Some(token) = tokens.peek() {
        if token.is_of_type(TokenType::And) {
            tokens.next();
            let right = parse_equality(tokens)?;
            let span = CodeSpan::new(expr.get_location().start, right.get_location().end);
            expr = Expression::BinaryOperation(Binary {
                operator: BinaryOperator::Conjunction,
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

fn parse_assignment(tokens: &mut TokenStream) -> Result<Expression> {
    let expr = parse_logic_or(tokens)?;

    if let Some(token) = tokens.peek() {
        if token.is_of_type(TokenType::Equal) {
            tokens.next();
            let init = parse_assignment(tokens)?;
            let span = CodeSpan::new(expr.get_location().start, init.get_location().end);
            return if let Expression::Identifier(ident) = expr {
                Ok(Expression::Assignment(Assignment {
                    ident,
                    expr: Box::new(init),
                    location: span,
                }))
            } else {
                Err(ParsingError::InvalidAssignmentTarget(span))
            };
        }
    }

    Ok(expr)
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
                parse_call(tokens)
            }
        }
    }
}

fn parse_call(tokens: &mut TokenStream) -> Result<Expression> {
    let mut expr = parse_primary(tokens)?;

    while let Some(token) = tokens.peek() {
        if token.is_of_type(TokenType::LeftParen) {
            tokens.next();
            let arguments = parse_arguments(tokens)?;
            let paren = consume(tokens, TokenType::RightParen)?;
            let span = expr.get_location();
            expr = Expression::Call(Call {
                callee: Box::new(expr),
                arguments,
                location: CodeSpan::combine(span, paren.get_span()),
            });
        } else {
            break;
        }
    }

    Ok(expr)
}

fn parse_arguments(tokens: &mut TokenStream) -> Result<Vec<Expression>> {
    if let Some(token) = tokens.peek() {
        if token.is_of_type(TokenType::RightParen) {
            Ok(Vec::new())
        } else {
            let mut arguments = vec![parse_expression(tokens)?];
            while let Some(next) = tokens.peek() {
                if next.is_of_type(TokenType::RightParen) {
                    break;
                } else if next.is_of_type(TokenType::Comma) {
                    tokens.next();
                    arguments.push(parse_expression(tokens)?);
                } else {
                    return Err(ParsingError::UnexpectedToken(next));
                }
            }
            if arguments.len() >= 255 {
                Err(ParsingError::TooManyArguments(CodeSpan::combine(
                    arguments.first().unwrap().get_location(),
                    arguments.last().unwrap().get_location(),
                )))
            } else {
                Ok(arguments)
            }
        }
    } else {
        Err(ParsingError::UnexpectedEndOfTokenStream(
            tokens.current_position(),
        ))
    }
}

fn parse_primary(tokens: &mut TokenStream) -> Result<Expression> {
    if let Some(token) = tokens.next() {
        let span = token.get_span();
        match token.consume() {
            TokenType::Identifier(s) => Ok(Expression::Identifier(Identifier {
                ident: s,
                location: span,
            })),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::tests::*;

    gen_tests!(
        primary,
        parse_primary,
        "true",
        "false",
        "nil",
        "\"hi\"",
        "42",
        "hello"
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

    gen_tests!(
        assignment,
        parse_assignment,
        "a = 1",
        "b = a + 1",
        "c = true == a != nil"
    );

    gen_tests!(
        multiple_assignments,
        parse_assignment,
        "a = b = 1",
        "c = d + (e = 3)"
    );

    gen_tests!(
        boolean_operators,
        parse_expression,
        "true or false",
        "entry and dessert",
        "a = true or false",
        "a or b or c"
    );

    gen_tests!(
        calls,
        parse_expression,
        "a()",
        "callback()()",
        "one_arg(1)",
        "lots_of_args(test(), \"a\", a == b)"
    );
}
