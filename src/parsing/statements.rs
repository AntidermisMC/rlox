use super::parsing_error::ParsingError;
use super::Result;
use crate::ast::statements::{Statement, Statements};
use crate::parsing::consume;
use crate::parsing::declarations::parse_declaration;
use crate::parsing::expressions::parse_expression;
use crate::scanning::{TokenStream, TokenType};

pub fn parse_declarations(tokens: &mut TokenStream) -> Vec<Statement> {
    let mut stmts = Vec::new();

    while let Ok(stmt) = parse_declaration(tokens) {
        stmts.push(stmt);
    }
    stmts
}

pub fn parse_statement(tokens: &mut TokenStream) -> Result<Statement> {
    match tokens.peek() {
        None => Err(ParsingError::UnexpectedEndOfTokenStream(
            tokens.current_position(),
        )),
        Some(t) => match t.get_type() {
            TokenType::Print => parse_print(tokens),
            TokenType::LeftBrace => {
                tokens.next();
                let stmts = parse_declarations(tokens);
                if let Some(rbrace) = tokens.next() {
                    if rbrace.is_of_type(TokenType::RightBrace) {
                        Ok(Statement::Block(Statements { stmts }))
                    } else {
                        Err(ParsingError::UnexpectedToken(rbrace))
                    }
                } else {
                    Err(ParsingError::UnexpectedEndOfTokenStream(
                        tokens.current_position(),
                    ))
                }
            }
            _ => {
                let expr = parse_expression(tokens)?;
                consume(tokens, TokenType::Semicolon)?;
                Ok(Statement::Expression(expr))
            }
        },
    }
}

fn parse_print(tokens: &mut TokenStream) -> Result<Statement> {
    if let Some(token) = tokens.next() {
        match token.get_type() {
            TokenType::Print => {
                let expr = parse_expression(tokens)?;
                consume(tokens, TokenType::Semicolon)?;
                Ok(Statement::Print(expr))
            }
            _ => Err(ParsingError::UnexpectedToken(token)),
        }
    } else {
        Err(ParsingError::UnexpectedEndOfTokenStream(
            tokens.current_position(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::*;
    use super::*;

    gen_tests!(
        test_print_statements,
        parse_print,
        "print 1;",
        "print 1 + 1;",
        "print \"hello\";"
    );

    gen_tests!(
        test_statements,
        parse_statement,
        "1;",
        "print 2;",
        "print 1 + 1;"
    );

    gen_tests!(
        test_blocks,
        parse_statement,
        "{\n}",
        "{\nprint 3;\n}",
        "{\nvar x = 42;\nprint x;\n}"
    );
}
