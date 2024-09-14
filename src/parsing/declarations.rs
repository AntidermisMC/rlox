use std::rc::Rc;

use super::Result;
use crate::{
    ast::{
        declarations::{ClassDeclaration, FunctionDeclaration, VariableDeclaration},
        expressions::{Expression, Identifier, Literal},
        statements::{Statement, Statements},
        types::Function,
        LiteralValue,
    },
    code_span::CodeSpan,
    parsing::{
        consume, parse_expression,
        statements::{parse_declarations, parse_statement},
        ParsingError,
    },
    scanning::{Token, TokenStream, TokenType},
};

pub fn parse_declaration(tokens: &mut TokenStream) -> Result<Statement> {
    if let Some(t) = tokens.peek() {
        match t.get_type() {
            TokenType::Var => {
                let var_dec = parse_variable_declaration(tokens)?;
                consume(tokens, TokenType::Semicolon)?;
                Ok(Statement::VariableDeclaration(var_dec))
            }
            TokenType::Fun => {
                let fun_dec = parse_function_declaration(tokens)?;
                Ok(Statement::FunctionDeclaration(fun_dec))
            }
            TokenType::Class => {
                let class_dec = parse_class_declaration(tokens)?;
                Ok(Statement::ClassDeclaration(class_dec))
            }
            _ => parse_statement(tokens),
        }
    } else {
        Err(ParsingError::UnexpectedEndOfTokenStream(
            tokens.current_position(),
        ))
    }
}

pub fn parse_class_declaration(tokens: &mut TokenStream) -> Result<ClassDeclaration> {
    consume(tokens, TokenType::Class)?;
    match tokens.next() {
        Some(token) => {
            let span = token.get_span();
            let mut methods = Vec::new();
            match token.consume() {
                TokenType::Identifier(name) => {
                    consume(tokens, TokenType::LeftBrace)?;
                    while tokens.peek().is_some_and(|t| t.is_identifier()) {
                        methods.push(parse_function(tokens)?);
                    }
                    consume(tokens, TokenType::RightBrace)?;
                    Ok(ClassDeclaration {
                        name: Identifier {
                            ident: name,
                            location: span,
                        },
                        methods,
                    })
                }
                token_type => Err(ParsingError::UnexpectedToken(Token::new(token_type, span))),
            }
        }
        None => Err(ParsingError::UnexpectedEndOfTokenStream(
            tokens.current_position(),
        )),
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

pub fn parse_function_declaration(tokens: &mut TokenStream) -> Result<FunctionDeclaration> {
    consume(tokens, TokenType::Fun)?;
    parse_function(tokens)
}

pub fn parse_function(tokens: &mut TokenStream) -> Result<FunctionDeclaration> {
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

                Ok(FunctionDeclaration {
                    name: Identifier {
                        ident: s.clone(),
                        location: span,
                    },
                    function: Rc::new(Function {
                        args: params,
                        body: Statements { stmts },
                        span,
                    }),
                })
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
    use super::{super::tests::*, *};

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
        "fun my_fun() {  }",
        "fun f(a) { print a;\n }",
        "fun g(a, b, c) { print a + b * c;\nprint \"hello\";\n }"
    );

    gen_tests!(
        test_class_declarations,
        parse_class_declaration,
        "class EmptyClass {\n}",
        "class OneMethod {\nempty_method() {  }\n}",
        "class TwoMethods {\nmethod_one() { return 2;\n }\nmethod_two(a) { print a;\n }\n}"
    );
}
