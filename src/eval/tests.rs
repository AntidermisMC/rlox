use crate::ast::expressions::ExpressionVisitor;
use crate::ast::statements::StatementVisitor;
use crate::ast::types::ValueType;
use crate::eval::out::OutputStream;
use crate::eval::Evaluator;
use crate::eval::ValueType::*;
use crate::parsing::{parse, parse_expression};
use crate::scanning::TokenStream;
use std::rc::Rc;

fn assert_eval_expr(code: &str, result: ValueType) {
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

macro_rules! gen_tests_expr {
    ($name:ident, $({ $case:expr, $expected:expr }),*) => {
        #[test]
        fn $name() {
            $(
            assert_eval_expr($case, $expected);
            )*
        }
    };
}

gen_tests_expr!(literals,
    { "1",          Number(1.0)                 },
    { "\"\"",       String(Rc::new("".to_string()))      },
    { "\"hello\"",  String(Rc::new("hello".to_string())) },
    { "nil",        Nil                         },
    { "true",       Boolean(true)               },
    { "false",      Boolean(false)              }
);

gen_tests_expr!(unary,
    { "-3",          Number(-3.0)   },
    { "--3",         Number(3.0)    },
    { "!true",       Boolean(false) },
    { "!false",      Boolean(true)  },
    { "!!true",      Boolean(true)  },
    { "!\"hello\"",  Boolean(false) },
    { "!nil",        Boolean(true)  }
);

gen_tests_expr!(string_concat,
    { r#""" + """#,             String(Rc::new("".to_string())) },
    { r#""Hello," + " World""#, String(Rc::new("Hello, World".to_string())) }
);

gen_tests_expr!(arithmetic_binary_operators,
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

gen_tests_expr!(comparison_binary_operators,
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

gen_tests_expr!(equality_same_types,
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

gen_tests_expr!(inequality_same_types,
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

gen_tests_expr!(complex_expressions,
    { r#""a" + "b" + "c""#, String(Rc::new("abc".to_string())) },
    { "!true != !false",    Boolean(true)             },
    { "4 + 3 == 14 / 2",    Boolean(true)             },
    { "1 + 2 * 3 == 7",     Boolean(true)             },
    { "(1 + 2) * 3 == 9",   Boolean(true)             },
    { " 1 < 2 == 3 >= 0",   Boolean(true)             }
);

fn assert_eval_stmts(code: &str, expected: &str) {
    let statements = parse(&mut TokenStream::new(code)).unwrap();
    let mut evaluator = Evaluator::new(OutputStream::File(std::string::String::new()));
    for stmt in &statements.stmts {
        evaluator.visit_statement(&stmt).unwrap();
    }
    if let OutputStream::File(s) = &evaluator.out {
        assert_eq!(s, expected);
    } else {
        assert!(false, "OutputStream is not a String !");
    }
}

macro_rules! gen_tests {
    ($ident:ident, $code:expr, $expected:expr) => {
        #[test]
        fn $ident() {
            assert_eval_stmts($code, $expected);
        }
    };
}

gen_tests!(
    print,
    r#"print "Hello World !";
    print 42;
    print true;
    print 1 + (2 * 3);"#,
    "Hello World !42true7"
);

gen_tests!(
    variables,
    "\
    var myvar = 1;
    var othervar = 2;
    var lastvar = myvar + othervar;
    print lastvar;
    ",
    "3"
);

gen_tests!(variables_no_assignment, "var myvar; print myvar;", "nil");

gen_tests!(
    variable_overwrite,
    "var myvar = 1; var myvar = 2; print myvar;",
    "2"
);

gen_tests!(variable_assignment, "var myvar; myvar = 3; print myvar; var othervar = 1; othervar = myvar + othervar; print othervar;", "34");

gen_tests!(blocks, "print 1; { print 2; } print 3;", "123");

gen_tests!(scope, "var a = 1; { print a; }", "1");

gen_tests!(
    variable_shadowing,
    "var a = 1; { var a = 2; print a; } print a;",
    "21"
);

gen_tests!(conditionals_true, "if (true) print \"a\";", "a");

gen_tests!(conditionals_false, "if (false) print \"a\";", "");

gen_tests!(
    conditionals_else_true,
    "if (true) print \"a\"; else print \"b\";",
    "a"
);

gen_tests!(
    conditionals_else_false,
    "if (false) print \"a\"; else print \"b\";",
    "b"
);

gen_tests!(
    conjunction_operator_simple,
    "print 1 and 1; print 1 and nil; print nil and 1; print nil and false;",
    "1nilnilnil"
);

gen_tests!(
    disjunction_operator_simple,
    "print 1 or 1; print 1 or nil; print nil or 1; print nil or false;",
    "111false"
);

gen_tests!(
    conjunction_operator_do_not_convert_to_boolean,
    "print nil and 1; print 2 and 3;",
    "nil3"
);

gen_tests!(
    disjunction_operator_do_not_convert_to_boolean,
    "print 1 or nil; print nil or 2;",
    "12"
);

gen_tests!(
    conjunction_operator_short_circuit,
    "var a = 1; var b = 3; true and (a = 2); false and (b = 4); print a * 10 + b;",
    "23"
);

gen_tests!(
    disjunction_operator_short_circuit,
    "var a = 1; var b = 3; true or (a = 2); false or (b = 4); print a * 10 + b;",
    "14"
);

gen_tests!(while_loop_false, "while (false) print 1;", "");

gen_tests!(
    while_loop_once,
    "var c = true; while (c) { print 1; c = false; }",
    "1"
);

gen_tests!(
    while_loop_many,
    "var i = 0; while (i < 10) i = i + 1; print i;",
    "10"
);

gen_tests!(
    for_loop,
    "for (var i = 0; i < 10; i = i + 1) print i;",
    "0123456789"
);

gen_tests!(
    for_loop_omitted_fields,
    "var i = 0; for (;i < 10;) { print i; i = i + 1; }",
    "0123456789"
);

gen_tests!(
    simple_fibonacci,
    "\
var a = 0;
var temp;

for (var b = 1; a < 10000; b = temp + b) {
  print a;
  temp = a;
  a = b;
}
",
    "011235813213455891442333776109871597258441816765"
);
