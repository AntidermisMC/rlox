use super::Result;
use crate::ast::declarations::{FunctionDeclaration, VariableDeclaration};
use crate::ast::expressions::{Expression, Identifier, Literal};
use crate::ast::statements::{Statement, Statements};
use crate::ast::types::Function;
use crate::ast::LiteralValue;
use crate::code_span::CodeSpan;
use crate::parsing::statements::{parse_declarations, parse_statement};
use crate::parsing::{consume, parse_expression, ParsingError};
use crate::scanning::{Token, TokenStream, TokenType};

pub fn parse_declaration(tokens: &mut TokenStream) -> Result<Statement> {
    if let Some(t) = tokens.peek() {
        match t.get_type() {
            TokenType::Var => {
                let var_dec = parse_variable_declaration(tokens)?;
                consume(tokens, TokenType::Semicolon)?;
                Ok(Statement::VariableDeclaration(var_dec))
            }
            TokenType::Fun => parse_function_declaration(tokens),
            _ => parse_statement(tokens),
        }
    } else {
        Err(ParsingError::UnexpectedEndOfTokenStream(
            tokens.current_position(),
        ))
    }
}

pub fn parse_variable_declaration(tokens: &mut TokenStream) -> Result<VariableDeclaration> {
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

pub fn parse_function_declaration(tokens: &mut TokenStream) -> Result<Statement> {
    consume(tokens, TokenType::Fun)?;

    match tokens.next() {
        Some(token) => {
            let span = token.get_span();
            if let TokenType::Identifier(s) = token.get_type() {
                consume(tokens, TokenType::LeftParen)?;
                let params = parse_parameters(tokens);
                consume(tokens, TokenType::RightParen)?;
                consume(tokens, TokenType::LeftBrace)?;
                let stmts = parse_declarations(tokens);
                consume(tokens, TokenType::RightBrace)?;

                Ok(Statement::FunctionDeclaration(FunctionDeclaration {
                    name: Identifier {
                        ident: s.clone(),
                        location: span,
                    },
                    function: Function {
                        args: params,
                        body: Statements { stmts },
                        span,
                    },
                }))
            } else {
                Err(ParsingError::UnexpectedToken(token))
            }
        }
        None => Err(ParsingError::UnexpectedEndOfTokenStream(
            tokens.current_position(),
        )),
    }
}

fn parse_parameters(tokens: &mut TokenStream) -> Vec<Identifier> {
    let mut params = Vec::<Identifier>::new();
    let mut save = tokens.save_position();

    while let Some(token) = tokens.next() {
        if let TokenType::Identifier(ident) = token.get_type() {
            params.push(Identifier {
                ident: ident.clone(),
                location: token.get_span(),
            });
            save = tokens.save_position();
            if let Some(t) = tokens.peek() {
                if let TokenType::Comma = t.get_type() {
                    tokens.next();
                    continue;
                }
            }
            break;
        } else {
            break;
        }
    }

    tokens.load_position(save);
    params
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

    gen_tests!(
        test_function_declarations,
        parse_declaration,
        //"fun my_fun() {  }",
        //"fun f(a) { print a;\n }",
        "fun g(a, b, c) { print a + b * c;\nprint \"hello\";\n }"
    );
}
