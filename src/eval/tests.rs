use crate::ast::expressions::ExpressionVisitor;
use crate::eval::out::OutputStream;
use crate::eval::types::ValueType::Number;
use crate::eval::types::{ValueType, ValueType::*};
use crate::eval::Evaluator;
use crate::parsing::parse_expression;
use crate::scanning::TokenStream;
use std::rc::Rc;

fn assert_eval(code: &str, result: ValueType) {
    let mut tokens = TokenStream::new(code);
    let tree = parse_expression(&mut tokens).unwrap();
    assert_eq!(
        Evaluator::new(OutputStream::File(std::string::String::new()))
            .visit_expression(&tree)
            .unwrap()
            .value,
        result
    );
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
    { "1",          Number(1.0)                 },
    { "\"\"",       String(Rc::new("".to_string()))      },
    { "\"hello\"",  String(Rc::new("hello".to_string())) },
    { "nil",        Nil                         },
    { "true",       Boolean(true)               },
    { "false",      Boolean(false)              }
);

gen_tests!(unary,
    { "-3",          Number(-3.0)   },
    { "--3",         Number(3.0)    },
    { "!true",       Boolean(false) },
    { "!false",      Boolean(true)  },
    { "!!true",      Boolean(true)  },
    { "!\"hello\"",  Boolean(false) },
    { "!nil",        Boolean(true)  }
);

gen_tests!(string_concat,
    { r#""" + """#,             String(Rc::new("".to_string())) },
    { r#""Hello," + " World""#, String(Rc::new("Hello, World".to_string())) }
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

gen_tests!(comparison_binary_operators,
    { "1 < 0",  Boolean(false) },
    { "1 < 1",  Boolean(false) },
    { "1 < 2",  Boolean(true)  },
    { "1 <= 0", Boolean(false) },
    { "1 <= 1", Boolean(true)  },
    { "1 <= 2", Boolean(true)  },
    { "1 > 0",  Boolean(true)  },
    { "1 > 1",  Boolean(false) },
    { "1 > 2",  Boolean(false) },
    { "1 >= 0", Boolean(true)  },
    { "1 >= 1", Boolean(true)  },
    { "1 >= 2", Boolean(false) }
);

gen_tests!(equality_same_types,
    { "1 == 1",              Boolean(true)  },
    { "1 == 2",              Boolean(false) },
    { "nil == nil",          Boolean(true)  },
    { "true == true",        Boolean(true)  },
    { "true == false",       Boolean(false) },
    { "false == true",       Boolean(false) },
    { "false == false",      Boolean(true)  },
    { r#""" == """#,         Boolean(true)  },
    { r#""hey" == "hey""#,   Boolean(true)  },
    { r#""hey" == """#,      Boolean(false) },
    { r#""hey" == "hello""#, Boolean(false) }
);

gen_tests!(inequality_same_types,
    { "1 != 1",              Boolean(false) },
    { "1 != 2",              Boolean(true)  },
    { "nil != nil",          Boolean(false) },
    { "true != true",        Boolean(false) },
    { "true != false",       Boolean(true)  },
    { "false != true",       Boolean(true)  },
    { "false != false",      Boolean(false) },
    { r#""" != """#,         Boolean(false) },
    { r#""hey" != "hey""#,   Boolean(false) },
    { r#""hey" != """#,      Boolean(true)  },
    { r#""hey" != "hello""#, Boolean(true)  }
);

gen_tests!(complex_expressions,
    { r#""a" + "b" + "c""#, String(Rc::new("abc".to_string())) },
    { "!true != !false",    Boolean(true)             },
    { "4 + 3 == 14 / 2",    Boolean(true)             },
    { "1 + 2 * 3 == 7",     Boolean(true)             },
    { "(1 + 2) * 3 == 9",   Boolean(true)             },
    { " 1 < 2 == 3 >= 0",   Boolean(true)             }
);

#[test]
fn variable_declaration() {
    let code = "";
}
