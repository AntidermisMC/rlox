use super::Result;
use crate::ast::declarations::VariableDeclaration;
use crate::ast::expressions::{Expression, Identifier, Literal};
use crate::ast::statements::Statement;
use crate::ast::LiteralValue;
use crate::code_span::CodeSpan;
use crate::parsing::statements::parse_statement;
use crate::parsing::{consume, parse_expression, ParsingError};
use crate::scanning::{Token, TokenStream, TokenType};

pub fn parse_declaration(tokens: &mut TokenStream) -> Result<Statement> {
    if let Some(t) = tokens.peek() {
        if t.consume() == TokenType::Var {
            Ok(Statement::VariableDeclaration(parse_variable_declaration(
                tokens,
            )?))
        } else {
            parse_statement(tokens)
        }
    } else {
        Err(ParsingError::UnexpectedEndOfTokenStream(
            tokens.current_position(),
        ))
    }
}

fn parse_variable_declaration(tokens: &mut TokenStream) -> Result<VariableDeclaration> {
    consume(tokens, TokenType::Var)?;
    match tokens.next() {
        Some(token) => {
            let position = token.get_span();
            match token.consume() {
                TokenType::Identifier(s) => {
                    let initializer = if consume(tokens, TokenType::Equal).is_ok() {
                        parse_expression(tokens)?
                    } else {
                        let location = tokens.current_position();
                        Expression::Literal(Literal::new(
                            LiteralValue::Nil,
                            CodeSpan::new(location, location),
                        ))
                    };
                    consume(tokens, TokenType::Semicolon)?;
                    Ok(VariableDeclaration {
                        name: Identifier {
                            ident: s,
                            location: position,
                        },
                        initializer,
                    })
                }
                token_type => Err(ParsingError::UnexpectedToken(Token::new(
                    token_type, position,
                ))),
            }
        }
        None => Err(ParsingError::UnexpectedEndOfTokenStream(
            tokens.current_position(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::*;
    use super::*;

    gen_tests!(
        test_variable_declarations,
        parse_variable_declaration,
        "var a = 1;",
        "var b;",
        "var c = 1 + 1 / 2;"
    );
}