use std::collections::HashSet;

use crate::ast::declarations::VariableDeclaration;
use runtime_error::RuntimeError;
use types::{Type, Value, ValueType};

use crate::ast::expressions::{
    Binary, BinaryOperator, Expression, ExpressionNode, ExpressionVisitor, Identifier, Literal,
    Unary, UnaryOperator,
};
use crate::ast::statements::{Statement, StatementVisitor};
use crate::ast::LiteralValue;
use crate::code_span::CodeSpan;
use crate::eval::RuntimeError::{DivisionByZero, MismatchedTypes};

mod runtime_error;
mod types;

#[cfg(test)]
mod tests;

pub struct Evaluator {}

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

fn as_string(value: &Value) -> Result<String> {
    match &value.value {
        ValueType::String(s) => Ok(s.clone()), // TODO remove cloning ?
        t => Err(MismatchedTypes(
            value.location,
            t.as_type(),
            HashSet::from([Type::String]),
        )),
    }
}

impl StatementVisitor for Evaluator {
    type Return = Result<()>;

    fn visit_statement(&self, stmt: &Statement) -> Self::Return {
        match stmt {
            Statement::Print(expr) => self.visit_print(expr),
            Statement::Expression(expr) => expr.accept(self).map(|_| ()),
            Statement::VariableDeclaration(declaration) => {
                self.visit_variable_declaration(declaration)
            }
        }
    }

    fn visit_print(&self, expr: &Expression) -> Self::Return {
        let value = expr.accept(self)?;
        print!("{}", value);
        Ok(())
    }

    fn visit_variable_declaration(&self, decl: &VariableDeclaration) -> Self::Return {
        todo!()
    }
}

impl ExpressionVisitor for Evaluator {
    type Return = Result<Value>;

    fn visit_literal(&self, literal: &Literal) -> Self::Return {
        Ok(match &literal.value {
            LiteralValue::StringLiteral(s) => {
                Value::new(ValueType::String(s.clone()), literal.location)
            }
            LiteralValue::NumberLiteral(n) => Value::new(ValueType::Number(*n), literal.location),
            LiteralValue::True => Value::new(ValueType::Boolean(true), literal.location),
            LiteralValue::False => Value::new(ValueType::Boolean(false), literal.location),
            LiteralValue::Nil => Value::new(ValueType::Nil, literal.location),
        })
    }

    fn visit_unary(&self, unary: &Unary) -> Self::Return {
        let value = self.visit_expression(unary.expr.as_ref())?;
        match (unary.op, value.value) {
            (UnaryOperator::Minus, ValueType::Number(n)) => {
                Ok(Value::new(ValueType::Number(-n), unary.location))
            }
            (UnaryOperator::Minus, v) => Err(MismatchedTypes(
                unary.location,
                Type::from(&v),
                HashSet::from([Type::Number]),
            )),
            (UnaryOperator::Not, val) => Ok(Value::new(
                ValueType::Boolean(!is_truthy(&val)),
                value.location,
            )),
        }
    }

    fn visit_binary(&self, binary: &Binary) -> Self::Return {
        let left = self.visit_expression(binary.left.as_ref())?;
        let right = self.visit_expression(binary.right.as_ref())?;
        let span = CodeSpan::combine(left.location, right.location);
        match binary.operator {
            BinaryOperator::Addition => addition(left, right, span),
            BinaryOperator::Subtraction => subtraction(left, right, span),
            BinaryOperator::Multiplication => multiplication(left, right, span),
            BinaryOperator::Division => division(left, right, span),
            BinaryOperator::StrictInferiority => strict_inferiority(left, right, span),
            BinaryOperator::Inferiority => inferiority(left, right, span),
            BinaryOperator::StrictSuperiority => strict_superiority(left, right, span),
            BinaryOperator::Superiority => superiority(left, right, span),
            BinaryOperator::Equality => equality(left, right, span),
            BinaryOperator::Inequality => inequality(left, right, span),
        }
    }

    fn visit_identifier(&self, identifier: &Identifier) -> Self::Return {
        todo!()
    }
}

fn addition(left: Value, right: Value, span: CodeSpan) -> Result<Value> {
    if let Ok(l) = as_number(&left) {
        if let Ok(r) = as_number(&right) {
            Ok(Value::new(ValueType::Number(l + r), span))
        } else {
            Err(MismatchedTypes(
                right.location,
                right.value.as_type(),
                HashSet::from([Type::Number]),
            ))
        }
    } else if let Ok(l) = as_string(&left) {
        if let Ok(r) = as_string(&right) {
            let mut l = l.clone();
            l.push_str(&r);
            Ok(Value::new(ValueType::String(l), span))
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

fn subtraction(left: Value, right: Value, span: CodeSpan) -> Result<Value> {
    Ok(Value::new(
        ValueType::Number(as_number(&left)? - as_number(&right)?),
        span,
    ))
}

fn multiplication(left: Value, right: Value, span: CodeSpan) -> Result<Value> {
    Ok(Value::new(
        ValueType::Number(as_number(&left)? * as_number(&right)?),
        span,
    ))
}

fn division(left: Value, right: Value, span: CodeSpan) -> Result<Value> {
    if right.value == ValueType::Number(0.0) {
        return Err(DivisionByZero(CodeSpan::combine(
            left.location,
            right.location,
        )));
    }
    Ok(Value::new(
        ValueType::Number(as_number(&left)? / as_number(&right)?),
        span,
    ))
}

fn strict_inferiority(left: Value, right: Value, span: CodeSpan) -> Result<Value> {
    Ok(Value::new(
        ValueType::Boolean(as_number(&left)? < as_number(&right)?),
        span,
    ))
}

fn strict_superiority(left: Value, right: Value, span: CodeSpan) -> Result<Value> {
    Ok(Value::new(
        ValueType::Boolean(as_number(&left)? > as_number(&right)?),
        span,
    ))
}

fn inferiority(left: Value, right: Value, span: CodeSpan) -> Result<Value> {
    Ok(Value::new(
        ValueType::Boolean(as_number(&left)? <= as_number(&right)?),
        span,
    ))
}

fn superiority(left: Value, right: Value, span: CodeSpan) -> Result<Value> {
    Ok(Value::new(
        ValueType::Boolean(as_number(&left)? >= as_number(&right)?),
        span,
    ))
}

fn test_equality(left: &ValueType, right: &ValueType) -> bool {
    match (left, right) {
        (ValueType::Boolean(l), ValueType::Boolean(r)) => l == r,
        (ValueType::Nil, ValueType::Nil) => true,
        (ValueType::Number(l), ValueType::Number(r)) => l == r,
        (ValueType::String(l), ValueType::String(r)) => l == r,
        (ValueType::Object(l), ValueType::Object(r)) => *l == *r,
        (_, _) => false,
    }
}

fn equality(left: Value, right: Value, span: CodeSpan) -> Result<Value> {
    let val = test_equality(&left.value, &right.value);
    Ok(Value::new(ValueType::Boolean(val), span))
}

fn inequality(left: Value, right: Value, span: CodeSpan) -> Result<Value> {
    let val = !test_equality(&left.value, &right.value);
    Ok(Value::new(ValueType::Boolean(val), span))
}
