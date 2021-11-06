mod expressions;
mod parsing_error;

use crate::ast::LiteralValue::{False, Nil, NumberLiteral, StringLiteral, True};
use crate::ast::{Binary, BinaryOperator, Expression, Literal, Unary, UnaryOperator};
use crate::code_span::CodeSpan;
use crate::scanning::TokenType;
use crate::scanning::{Token, TokenStream};
use expressions::parse_expression;
pub use parsing_error::ParsingError;
use std::convert::TryFrom;

type Result<T> = std::result::Result<T, ParsingError>;

pub fn parse(tokens: &mut TokenStream) -> Result<Expression> {
    parse_expression(tokens)
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

pub(crate) use try_parse;

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::ast::Expression;
    use crate::scanning::TokenStream;

    pub fn assert_equal_repr(
        to_be_tested: &str,
        parsing_function: fn(&mut TokenStream) -> Result<Expression>,
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
}
