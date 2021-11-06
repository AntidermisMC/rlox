mod expressions;
mod parsing_error;
mod statements;

use crate::scanning::TokenStream;
pub use parsing_error::ParsingError;

type Result<T> = std::result::Result<T, ParsingError>;

pub fn parse(tokens: &mut TokenStream) -> Result<Statements> {
    let mut stmts = Vec::new();

    while tokens.has_next() {
        stmts.push(parse_statement(tokens)?);
    }

    Ok(Statements { stmts })
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

use crate::ast::statements::Statements;
use crate::parsing::statements::parse_statement;
pub(crate) use try_parse;

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
}
