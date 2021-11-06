use super::parsing_error::ParsingError;
use super::Result;
use crate::ast::statements::Statement;
use crate::parsing::expressions::parse_expression;
use crate::scanning::{TokenStream, TokenType};

pub fn parse_statements(tokens: &mut TokenStream) -> Vec<Statement> {
    let mut stmts = Vec::new();

    while let Ok(stmt) = parse_statement(tokens) {
        stmts.push(stmt);
    }
    stmts
}

pub fn parse_statement(tokens: &mut TokenStream) -> Result<Statement> {
    match tokens.peek() {
        None => Err(ParsingError::UnexpectedEndOfTokenStream(
            tokens.current_position(),
        )),
        Some(t) => {
            if let TokenType::Print = t.get_type() {
                parse_print(tokens)
            } else {
                Ok(Statement::Expression(parse_expression(tokens)?))
            }
        }
    }
}

fn parse_print(tokens: &mut TokenStream) -> Result<Statement> {
    if let Some(token) = tokens.next() {
        match token.get_type() {
            TokenType::Print => Ok(Statement::Print(parse_expression(tokens)?)),
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
        "print 1",
        "print 1 + 1",
        "print \"hello\""
    );

    gen_tests!(
        test_statements,
        parse_statement,
        "1",
        "print 2",
        "print 1 + 1"
    );
}
