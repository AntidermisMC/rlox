mod conversions;
pub mod expressions;

use crate::code_span::CodeSpan;
use expressions::*;
use expressions::{Binary, Expression, Literal, Unary};
use std::fmt::{Display, Formatter};

pub enum LiteralValue {
    StringLiteral(String),
    NumberLiteral(f64),
    True,
    False,
    Nil,
}

// ########## Visitor stuff

pub trait AstNode {
    fn accept<T: AstVisitor>(&self, visitor: &T) -> T::Return;
}

pub trait AstVisitor: Sized {
    type Return;

    fn visit_expr(&self, expr: &Expression) -> Self::Return {
        match expr {
            Expression::Literal(l) => l.accept(self),
            Expression::UnaryOperation(u) => u.accept(self),
            Expression::BinaryOperation(b) => b.accept(self),
        }
    }

    fn visit_literal(&self, literal: &Literal) -> Self::Return;
    fn visit_unary(&self, unary: &Unary) -> Self::Return;
    fn visit_binary(&self, binary: &Binary) -> Self::Return;
}

// ############## Display stuff
