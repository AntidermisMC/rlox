use crate::ast::AstVisitor;
use crate::eval::types::ValueType::Number;
use crate::eval::types::{ValueType, ValueType::*};
use crate::eval::Evaluator;
use crate::parsing::parse;
use crate::scanning::TokenStream;

fn assert_eval(code: &str, result: ValueType) {
    let mut tokens = TokenStream::new(code);
    let tree = parse(&mut tokens).unwrap();
    let value = Evaluator {}.visit_expr(&tree).unwrap();
    assert_eq!(value.value, result);
}

macro_rules! gen_tests {
    ($name:ident, $({ $case:expr, $expected:expr }),*) => {
        #[test]
        fn $name() {
            $(
            assert_eval($case, $expected);
            )*
        }
    };
}

gen_tests!(literals,
    { "1",          Number(1.0) },
    { "\"\"",       String("".to_string()) },
    { "\"hello\"",  String("hello".to_string()) },
    { "nil",        Nil },
    { "true",       Boolean(true) },
    { "false",      Boolean(false) }
);

gen_tests!(unary,
    { "-3",          Number(-3.0) },
    { "--3",         Number(3.0) },
    { "!true",       Boolean(false) },
    { "!false",      Boolean(true) },
    { "!!true",      Boolean(true) },
    { "!\"hello\"",  Boolean(false) },
    { "!nil",        Boolean(true) }
);

gen_tests!(string_concat,
    { r#""" + """#,             String("".to_string()) },
    { r#""Hello," + " World""#, String("Hello, World".to_string()) }
);

gen_tests!(arithmetic_binary_operators,
    { "1 + 1", Number(2.0) },
    { "1 - 1", Number(0.0) },
    { "1 * 1", Number(1.0) },
    { "1 / 1", Number(1.0) },
    { "2 + 2", Number(4.0) },
    { "2 - 2", Number(0.0) },
    { "2 * 2", Number(4.0) },
    { "2 / 2", Number(1.0) },
    { "0 / 1", Number(0.0) }
);
