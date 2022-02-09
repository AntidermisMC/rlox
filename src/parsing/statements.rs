use super::parsing_error::ParsingError;
use super::Result;
use crate::ast::expressions::{Expression, Literal};
use crate::ast::statements::{Conditional, ForLoop, Statement, Statements, WhileLoop};
use crate::ast::LiteralValue;
use crate::code_span::CodeSpan;
use crate::parsing::consume;
use crate::parsing::declarations::{parse_declaration, parse_variable_declaration};
use crate::parsing::expressions::parse_expression;
use crate::scanning::{TokenStream, TokenType};

pub fn parse_declarations(tokens: &mut TokenStream) -> Vec<Statement> {
    let mut stmts = Vec::new();

    let mut save = tokens.save_position();
    while let Ok(stmt) = parse_declaration(tokens) {
        stmts.push(stmt);
        save = tokens.save_position()
    }
    tokens.load_position(save);
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
                let stmts = parse_declarations(tokens); // TODO error here
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
            TokenType::If => parse_conditional(tokens),
            TokenType::While => parse_while_loop(tokens),
            TokenType::For => parse_for(tokens),
            TokenType::Return => parse_return(tokens),
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

fn parse_conditional(tokens: &mut TokenStream) -> Result<Statement> {
    if let Some(token) = tokens.peek() {
        match token.get_type() {
            TokenType::If => {
                tokens.next();
                consume(tokens, TokenType::LeftParen)?;
                let condition = parse_expression(tokens)?;
                consume(tokens, TokenType::RightParen)?;
                let then_statement = parse_statement(tokens)?;
                let else_statement = if tokens
                    .peek()
                    .map(|token| token.is_of_type(TokenType::Else))
                    .unwrap_or(false)
                {
                    tokens.next();
                    Some(parse_statement(tokens)?)
                } else {
                    None
                };
                Ok(Statement::Conditional(Box::new(Conditional {
                    condition,
                    then_statement,
                    else_statement,
                })))
            }
            _ => Err(ParsingError::UnexpectedToken(token)),
        }
    } else {
        Err(ParsingError::UnexpectedEndOfTokenStream(
            tokens.current_position(),
        ))
    }
}

fn parse_while_loop(tokens: &mut TokenStream) -> Result<Statement> {
    if let Some(token) = tokens.peek() {
        match token.get_type() {
            TokenType::While => {
                tokens.next();
                consume(tokens, TokenType::LeftParen)?;
                let condition = parse_expression(tokens)?;
                consume(tokens, TokenType::RightParen)?;
                let statement = parse_statement(tokens)?;
                Ok(Statement::WhileLoop(Box::new(WhileLoop {
                    condition,
                    statement,
                })))
            }
            _ => Err(ParsingError::UnexpectedToken(token)),
        }
    } else {
        Err(ParsingError::UnexpectedEndOfTokenStream(
            tokens.current_position(),
        ))
    }
}

fn parse_for(tokens: &mut TokenStream) -> Result<Statement> {
    if let Some(token) = tokens.peek() {
        match token.get_type() {
            TokenType::For => {
                tokens.next();
                consume(tokens, TokenType::LeftParen)?;

                let initializer = if tokens
                    .peek()
                    .map(|t| t.is_of_type(TokenType::Semicolon))
                    .unwrap_or(false)
                {
                    None
                } else if tokens
                    .peek()
                    .map(|t| t.is_of_type(TokenType::Var))
                    .unwrap_or(false)
                {
                    Some(Statement::VariableDeclaration(parse_variable_declaration(
                        tokens,
                    )?))
                } else {
                    Some(Statement::Expression(parse_expression(tokens)?))
                };
                consume(tokens, TokenType::Semicolon)?;

                let condition = if tokens
                    .peek()
                    .map(|t| t.is_of_type(TokenType::Semicolon))
                    .unwrap_or(false)
                {
                    None
                } else {
                    Some(parse_expression(tokens)?)
                };
                consume(tokens, TokenType::Semicolon)?;

                let increment = if tokens
                    .peek()
                    .map(|t| t.is_of_type(TokenType::RightParen))
                    .unwrap_or(false)
                {
                    None
                } else {
                    Some(parse_expression(tokens)?)
                };

                consume(tokens, TokenType::RightParen)?;
                let body = parse_statement(tokens)?;
                Ok(Statement::ForLoop(Box::new(ForLoop {
                    initializer,
                    condition,
                    increment,
                    body,
                })))
            }
            _ => Err(ParsingError::UnexpectedToken(token)),
        }
    } else {
        Err(ParsingError::UnexpectedEndOfTokenStream(
            tokens.current_position(),
        ))
    }
}

fn parse_return(tokens: &mut TokenStream) -> Result<Statement> {
    consume(tokens, TokenType::Return)?;
    let expr = if tokens
        .peek()
        .map(|t| t.is_of_type(TokenType::Semicolon))
        .unwrap_or(true)
    {
        Expression::Literal(Literal {
            value: LiteralValue::Nil,
            location: CodeSpan::new(tokens.current_position(), tokens.current_position()),
        })
    } else {
        parse_expression(tokens)?
    };
    consume(tokens, TokenType::Semicolon)?;
    Ok(Statement::Return(expr))
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
        test_statement,
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

    gen_tests!(
        test_conditionals,
        parse_statement,
        "if (true) {\nfalse;\n}",
        "if (false) true; else \"Something else\";",
        "{\nif (true) something;\nprint something_else;\n}"
    );

    gen_tests!(
        test_while_loop,
        parse_statement,
        "while (true) print a;",
        "while (true) {\n}"
    );

    gen_tests!(
        test_for_loop,
        parse_statement,
        "for (;;) print 1;",
        "for (var i = 0; i < 10; i = i + 1) print i;"
    );

    gen_tests!(
        test_return,
        parse_statement,
        "return;",
        "return myvar;",
        "return 1 + 2;"
    );

    #[test]
    fn test_statements() {
        let parsed = parse_declarations(&mut TokenStream::new("var a = 1;\n print a;\n"));
        let stmts = Statements { stmts: parsed };
        assert_eq!("var a = 1;\nprint a;\n", stmts.to_string());
    }
}
