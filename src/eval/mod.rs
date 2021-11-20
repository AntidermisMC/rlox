use std::collections::HashSet;
use std::fmt::Write;
use std::rc::Rc;

use crate::ast::declarations::VariableDeclaration;
use runtime_error::RuntimeError;
use types::{Type, Value, ValueType};

use crate::ast::expressions::{
    Assignment, Binary, BinaryOperator, Expression, ExpressionNode, ExpressionVisitor, Identifier,
    Literal, Unary, UnaryOperator,
};
use crate::ast::statements::{Statement, StatementVisitor};
use crate::ast::LiteralValue;
use crate::code_span::CodeSpan;
use crate::eval::environment::Environment;
use crate::eval::out::OutputStream;
use crate::eval::RuntimeError::{DivisionByZero, MismatchedTypes};

mod environment;
pub mod out;
mod runtime_error;
mod types;

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

impl StatementVisitor for Evaluator {
    type Return = Result<()>;

    fn visit_statement(&mut self, stmt: &Statement) -> Self::Return {
        match stmt {
            Statement::Print(expr) => self.visit_print(expr),
            Statement::Expression(expr) => expr.accept(self).map(|_| ()),
            Statement::VariableDeclaration(declaration) => {
                self.visit_variable_declaration(declaration)
            }
            Statement::Block(stmts) => {
                self.env.push_env();
                for stmt in &stmts.stmts {
                    self.visit_statement(&stmt)?;
                }
                self.env.pop_env();
                Ok(())
            }
        }
    }

    fn visit_print(&mut self, expr: &Expression) -> Self::Return {
        let value = expr.accept(self)?;
        write!(self.out, "{}", value).map_err(|_| RuntimeError::WriteError(expr.get_location()))?;
        Ok(())
    }

    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration) -> Self::Return {
        let init = self.visit_expression(&decl.initializer)?;
        self.env.define(decl.name.ident.to_string(), init.value);
        Ok(())
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
        let right = self.visit_expression(binary.right.as_ref())?;
        let value_type = match binary.operator {
            BinaryOperator::Addition => addition(left, right),
            BinaryOperator::Subtraction => subtraction(left, right),
            BinaryOperator::Multiplication => multiplication(left, right),
            BinaryOperator::Division => division(left, right),
            BinaryOperator::StrictInferiority => strict_inferiority(left, right),
            BinaryOperator::Inferiority => inferiority(left, right),
            BinaryOperator::StrictSuperiority => strict_superiority(left, right),
            BinaryOperator::Superiority => superiority(left, right),
            BinaryOperator::Equality => equality(left, right),
            BinaryOperator::Inequality => inequality(left, right),
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
        (ValueType::Object(l), ValueType::Object(r)) => *l == *r,
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
