use super::Result;
use crate::ast::declarations::VariableDeclaration;
use crate::ast::expressions::{Expression, Identifier, Literal};
use crate::ast::LiteralValue;
use crate::code_span::CodeSpan;
use crate::parsing::{consume, parse_expression, ParsingError};
use crate::scanning::{Token, TokenStream, TokenType};

pub fn parse_declaration(tokens: &mut TokenStream) -> Result<()> {
    todo!()
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
