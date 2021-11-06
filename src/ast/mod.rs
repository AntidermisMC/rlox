mod conversions;
pub mod expressions;
pub mod statements;

use crate::ast::statements::Statement;
use expressions::{Binary, Expression, Literal, Unary};

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

    fn visit_expression(&self, expr: &Expression) -> Self::Return {
        match expr {
            Expression::Literal(l) => l.accept(self),
            Expression::UnaryOperation(u) => u.accept(self),
            Expression::BinaryOperation(b) => b.accept(self),
        }
    }

    fn visit_statement(&self, stmt: &Statement) -> Self::Return {
        match stmt {
            Statement::Print(expr) => expr.accept(self),
            Statement::Expression(expr) => expr.accept(self),
        }
    }

    fn visit_literal(&self, literal: &Literal) -> Self::Return;
    fn visit_unary(&self, unary: &Unary) -> Self::Return;
    fn visit_binary(&self, binary: &Binary) -> Self::Return;
}

// ############## Display stuff
