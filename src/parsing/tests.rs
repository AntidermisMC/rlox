use super::*;

fn assert_equal_repr(
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
    };
}

gen_tests!(
    primary,
    parse_primary,
    "true",
    "false",
    "nil",
    "\"hi\"",
    "42"
);

gen_tests!(
    unary,
    parse_unary,
    "true",
    "!true",
    "!!true",
    "-1",
    "--1",
    "!(1 + 1)"
);

gen_tests!(
    factor,
    parse_factor,
    "1",
    "1 * 1",
    "1 / 1",
    "1 * 1 / 1",
    "1 * (1 / 1)",
    "-(1 / 1)"
);

gen_tests!(
    term,
    parse_term,
    "1",
    "1 + 1",
    "1 - 1",
    "1 + 1 - 1",
    "1 + (1 - 1)",
    "1 * 1 + 1",
    "1 - 1 * 1"
);

gen_tests!(
    comparison,
    parse_comparison,
    "1",
    "1 < 1",
    "1 * 1 > 1",
    "-1 <= 1 + 1",
    "!(1 >= 1)"
);

gen_tests!(
    equality,
    parse_equality,
    "1",
    "1 == 1",
    "1 + 1 != 1",
    "true == !false"
);
