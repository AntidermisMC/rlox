mod declarations;
mod expressions;
mod parsing_error;
mod statements;

use crate::ast::statements::Statements;
use crate::parsing::declarations::parse_declaration;
use crate::scanning::{Token, TokenStream, TokenType};
pub use expressions::parse_expression;
pub use parsing_error::ParsingError;

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
pub(crate) use try_parse;

type Result<T> = std::result::Result<T, ParsingError>;

pub fn parse(tokens: &mut TokenStream) -> Result<Statements> {
    let mut stmts = Vec::new();

    while tokens.has_next() {
        stmts.push(parse_declaration(tokens)?);
    }

    Ok(Statements { stmts })
}

/// Consumes the first token of the stream if it is of the right type, else errors.
#[must_use]
fn consume(tokens: &mut TokenStream, token: TokenType) -> Result<Token> {
    match tokens.peek() {
        Some(t) if t.is_of_type(token) => {
            tokens.next();
            Ok(t)
        }
        Some(t) => Err(ParsingError::UnexpectedToken(t)),
        None => Err(ParsingError::UnexpectedEndOfTokenStream(
            tokens.current_position(),
        )),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::scanning::TokenStream;

    pub fn assert_equal_repr<T: ToString>(
        to_be_tested: &str,
        parsing_function: fn(&mut TokenStream) -> Result<T>,
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
    }
    }

    pub(crate) use gen_tests;
    pub(crate) use test_cases;

    gen_tests!(
        multiple_statements,
        parse,
        "print 1;\nprint 2;\n",
        "1;\nprint 2;\n"
    );
}
