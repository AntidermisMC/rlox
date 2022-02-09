use std::collections::HashSet;
use std::rc::Rc;

pub use builtins::prelude;
use runtime_error::RuntimeError;

use crate::ast::expressions::{
    Assignment, Binary, BinaryOperator, Call, Expression, ExpressionVisitor, Identifier, Literal,
    Unary, UnaryOperator,
};
use crate::ast::statements::StatementVisitor;
use crate::ast::types::{NativeFunction, Object, Type, Value, ValueType};
use crate::ast::LiteralValue;
use crate::code_span::CodeSpan;
use crate::eval::environment::Environment;
use crate::eval::output_stream::OutputStream;
use crate::eval::RuntimeError::{DivisionByZero, MismatchedTypes};

mod builtins;
mod environment;
pub mod output_stream;
mod runtime_error;
mod statements;

#[cfg(test)]
mod tests;

pub struct Evaluator {
    env: Environment,
    out: OutputStream,
}

impl Evaluator {
    pub fn new(out: OutputStream) -> Self {
        Evaluator {
            env: Environment::new(),
            out,
        }
    }

    pub fn register_prelude(&mut self, prelude: Vec<(&str, NativeFunction, usize)>) {
        for (name, function, arity) in prelude {
            self.env
                .define(name.to_string(), ValueType::NativeFunction(function, arity));
        }
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

fn is_truthy(value: &ValueType) -> bool {
    match value {
        ValueType::Boolean(false) | ValueType::Nil => false,
        _ => true,
    }
}

fn as_number(value: &Value) -> Result<f64> {
    match value.value {
        ValueType::Number(n) => Ok(n),
        _ => Err(MismatchedTypes(
            value.location,
            Type::Number,
            HashSet::from([Type::Number]),
        )),
    }
}

fn as_string(value: &Value) -> Result<Rc<String>> {
    match &value.value {
        ValueType::String(s) => Ok(s.clone()),
        t => Err(MismatchedTypes(
            value.location,
            t.as_type(),
            HashSet::from([Type::String]),
        )),
    }
}

impl ExpressionVisitor for Evaluator {
    type Return = Result<Value>;

    fn visit_literal(&mut self, literal: &Literal) -> Self::Return {
        let value = (*literal).clone();
        let value_type = match value.value {
            LiteralValue::StringLiteral(s) => ValueType::String(Rc::new(s)),
            LiteralValue::NumberLiteral(n) => ValueType::Number(n),
            LiteralValue::True => ValueType::Boolean(true),
            LiteralValue::False => ValueType::Boolean(false),
            LiteralValue::Nil => ValueType::Nil,
        };
        Ok(Value::new(value_type, literal.location))
    }

    fn visit_unary(&mut self, unary: &Unary) -> Self::Return {
        let operand = self.visit_expression(unary.expr.as_ref())?;
        let value_type = match (unary.op, operand.value) {
            (UnaryOperator::Minus, ValueType::Number(n)) => Ok(ValueType::Number(-n)),
            (UnaryOperator::Minus, v) => Err(MismatchedTypes(
                unary.location,
                Type::from(&v),
                HashSet::from([Type::Number]),
            )),
            (UnaryOperator::Not, val) => Ok(ValueType::Boolean(!is_truthy(&val))),
        };
        Ok(Value::new(value_type?, unary.location))
    }

    fn visit_binary(&mut self, binary: &Binary) -> Self::Return {
        let left = self.visit_expression(binary.left.as_ref())?;
        let value_type = match binary.operator {
            BinaryOperator::Addition => {
                addition(left, self.visit_expression(binary.right.as_ref())?)
            }
            BinaryOperator::Subtraction => {
                subtraction(left, self.visit_expression(binary.right.as_ref())?)
            }
            BinaryOperator::Multiplication => {
                multiplication(left, self.visit_expression(binary.right.as_ref())?)
            }
            BinaryOperator::Division => {
                division(left, self.visit_expression(binary.right.as_ref())?)
            }
            BinaryOperator::StrictInferiority => {
                strict_inferiority(left, self.visit_expression(binary.right.as_ref())?)
            }
            BinaryOperator::Inferiority => {
                inferiority(left, self.visit_expression(binary.right.as_ref())?)
            }
            BinaryOperator::StrictSuperiority => {
                strict_superiority(left, self.visit_expression(binary.right.as_ref())?)
            }
            BinaryOperator::Superiority => {
                superiority(left, self.visit_expression(binary.right.as_ref())?)
            }
            BinaryOperator::Equality => {
                equality(left, self.visit_expression(binary.right.as_ref())?)
            }
            BinaryOperator::Inequality => {
                inequality(left, self.visit_expression(binary.right.as_ref())?)
            }
            BinaryOperator::Disjunction => disjunction(left, binary.right.as_ref(), self),
            BinaryOperator::Conjunction => conjunction(left, binary.right.as_ref(), self),
        };
        Ok(Value::new(value_type?, binary.location))
    }

    fn visit_identifier(&mut self, identifier: &Identifier) -> Self::Return {
        match self.env.get(&identifier.ident) {
            Some(value) => Ok(Value::new(value.clone(), identifier.location)),
            None => Err(RuntimeError::UnboundName(
                identifier.location,
                identifier.ident.to_string(),
            )),
        }
    }

    fn visit_assignment(&mut self, assignment: &Assignment) -> Self::Return {
        let expr = self.visit_expression(&assignment.expr)?;
        self.env
            .assign(assignment.ident.ident.clone(), expr.clone())?;
        Ok(expr)
    }

    fn visit_call(&mut self, call: &Call) -> Self::Return {
        let callee = self.visit_expression(call.callee.as_ref())?;

        let mut arguments = Vec::new();
        for argument in &call.arguments {
            arguments.push(self.visit_expression(argument)?.value)
        }

        match callee.value {
            ValueType::NativeFunction(f, arity) => {
                if arguments.len() != arity {
                    Err(RuntimeError::InvalidArgumentCount(
                        call.location,
                        arity,
                        arguments.len(),
                    ))
                } else {
                    Ok(Value {
                        value: f(arguments, call.location)?,
                        location: call.location,
                    })
                }
            }
            ValueType::Function(f) => {
                if arguments.len() != f.args.len() {
                    Err(RuntimeError::InvalidArgumentCount(
                        call.location,
                        f.args.len(),
                        arguments.len(),
                    ))
                } else {
                    self.env.push_env();
                    for (arg, value) in f.args.iter().zip(arguments) {
                        self.env.define(arg.ident.clone(), value);
                    }
                    for stmt in &f.body.stmts {
                        self.visit_statement(stmt)?;
                    }
                    self.env.pop_env();

                    Ok(Value {
                        location: f.span,
                        value: ValueType::Nil,
                    })
                }
            }
            _ => Err(RuntimeError::NotCallable(callee.location)),
        }
    }
}

fn addition(left: Value, right: Value) -> Result<ValueType> {
    if let Ok(l) = as_number(&left) {
        if let Ok(r) = as_number(&right) {
            Ok(ValueType::Number(l + r))
        } else {
            Err(MismatchedTypes(
                right.location,
                right.value.as_type(),
                HashSet::from([Type::Number]),
            ))
        }
    } else if let Ok(l) = as_string(&left) {
        if let Ok(r) = as_string(&right) {
            let mut l = (*l).clone();
            l.push_str(&r);
            Ok(ValueType::String(Rc::new(l)))
        } else {
            Err(MismatchedTypes(
                right.location,
                right.value.as_type(),
                HashSet::from([Type::String]),
            ))
        }
    } else {
        Err(MismatchedTypes(
            left.location,
            left.value.as_type(),
            HashSet::from([Type::Number, Type::String]),
        ))
    }
}

fn subtraction(left: Value, right: Value) -> Result<ValueType> {
    Ok(ValueType::Number(as_number(&left)? - as_number(&right)?))
}

fn multiplication(left: Value, right: Value) -> Result<ValueType> {
    Ok(ValueType::Number(as_number(&left)? * as_number(&right)?))
}

fn division(left: Value, right: Value) -> Result<ValueType> {
    if right.value == ValueType::Number(0.0) {
        return Err(DivisionByZero(CodeSpan::combine(
            left.location,
            right.location,
        )));
    }
    Ok(ValueType::Number(as_number(&left)? / as_number(&right)?))
}

fn strict_inferiority(left: Value, right: Value) -> Result<ValueType> {
    Ok(ValueType::Boolean(as_number(&left)? < as_number(&right)?))
}

fn strict_superiority(left: Value, right: Value) -> Result<ValueType> {
    Ok(ValueType::Boolean(as_number(&left)? > as_number(&right)?))
}

fn inferiority(left: Value, right: Value) -> Result<ValueType> {
    Ok(ValueType::Boolean(as_number(&left)? <= as_number(&right)?))
}

fn superiority(left: Value, right: Value) -> Result<ValueType> {
    Ok(ValueType::Boolean(as_number(&left)? >= as_number(&right)?))
}

fn test_equality(left: &Value, right: &Value) -> bool {
    match (&left.value, &right.value) {
        (ValueType::Boolean(l), ValueType::Boolean(r)) => l == r,
        (ValueType::Nil, ValueType::Nil) => true,
        (ValueType::Number(l), ValueType::Number(r)) => l == r,
        (ValueType::String(l), ValueType::String(r)) => l == r,
        (ValueType::Object(Object::Object(l)), ValueType::Object(Object::Object(r))) => *l == *r,
        (_, _) => false,
    }
}

fn equality(left: Value, right: Value) -> Result<ValueType> {
    let val = test_equality(&left, &right);
    Ok(ValueType::Boolean(val))
}

fn inequality(left: Value, right: Value) -> Result<ValueType> {
    let val = !test_equality(&left, &right);
    Ok(ValueType::Boolean(val))
}

fn disjunction(left: Value, right: &Expression, visitor: &mut Evaluator) -> Result<ValueType> {
    if is_truthy(&left.value) {
        Ok(left.value)
    } else {
        visitor.visit_expression(&right).map(|val| val.value)
    }
}

fn conjunction(left: Value, right: &Expression, visitor: &mut Evaluator) -> Result<ValueType> {
    if !is_truthy(&left.value) {
        Ok(left.value)
    } else {
        visitor.visit_expression(&right).map(|val| val.value)
    }
}
