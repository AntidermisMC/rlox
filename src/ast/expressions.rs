use std::fmt::{Display, Formatter};

use crate::ast::LiteralValue;
use crate::code_span::CodeSpan;

pub enum Expression {
    Literal(Literal),
    UnaryOperation(Unary),
    BinaryOperation(Binary),
    Identifier(Identifier),
    Assignment(Assignment),
}

#[derive(Clone)]
pub struct Literal {
    pub value: LiteralValue,
    pub location: CodeSpan,
}

pub struct Unary {
    pub op: UnaryOperator,
    pub expr: Box<Expression>,
    pub location: CodeSpan,
}

#[derive(Copy, Clone)]
pub enum UnaryOperator {
    Minus,
    Not,
}

pub struct Binary {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub location: CodeSpan,
}

#[derive(Copy, Clone)]
pub enum BinaryOperator {
    Equality,
    Inequality,
    StrictInferiority,
    Inferiority,
    StrictSuperiority,
    Superiority,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Conjunction,
    Disjunction,
}

pub struct Assignment {
    pub ident: Identifier,
    pub expr: Box<Expression>,
    pub location: CodeSpan,
}

pub struct Identifier {
    pub ident: String,
    pub location: CodeSpan,
}

impl Expression {
    pub fn get_location(&self) -> CodeSpan {
        match self {
            Expression::Literal(l) => l.location,
            Expression::UnaryOperation(u) => u.location,
            Expression::BinaryOperation(b) => b.location,
            Expression::Identifier(i) => i.location,
            Expression::Assignment(a) => a.location,
        }
    }
}

impl Literal {
    pub fn new(value: LiteralValue, location: CodeSpan) -> Self {
        Self { value, location }
    }
}

pub trait Priority {
    fn priority(&self) -> u8;
}

impl Priority for BinaryOperator {
    fn priority(&self) -> u8 {
        match self {
            BinaryOperator::Equality => 1,
            BinaryOperator::Inequality => 1,
            BinaryOperator::Conjunction => 2,
            BinaryOperator::Disjunction => 2,
            BinaryOperator::StrictInferiority => 3,
            BinaryOperator::Inferiority => 3,
            BinaryOperator::StrictSuperiority => 3,
            BinaryOperator::Superiority => 3,
            BinaryOperator::Addition => 4,
            BinaryOperator::Subtraction => 4,
            BinaryOperator::Multiplication => 5,
            BinaryOperator::Division => 5,
        }
    }
}

impl Priority for Binary {
    fn priority(&self) -> u8 {
        self.operator.priority()
    }
}

impl Priority for UnaryOperator {
    fn priority(&self) -> u8 {
        6
    }
}

impl Priority for Unary {
    fn priority(&self) -> u8 {
        self.op.priority()
    }
}

impl Priority for Literal {
    fn priority(&self) -> u8 {
        7
    }
}

impl Priority for Identifier {
    fn priority(&self) -> u8 {
        7
    }
}

impl Priority for Assignment {
    fn priority(&self) -> u8 {
        0
    }
}

impl Priority for Expression {
    fn priority(&self) -> u8 {
        match self {
            Expression::Literal(l) => l.priority(),
            Expression::UnaryOperation(u) => u.priority(),
            Expression::BinaryOperation(b) => b.operator.priority(),
            Expression::Identifier(i) => i.priority(),
            Expression::Assignment(a) => a.priority(),
        }
    }
}

impl ExpressionNode for Expression {
    fn accept<T: ExpressionVisitor>(&self, visitor: &mut T) -> T::Return {
        visitor.visit_expression(self)
    }
}

impl ExpressionNode for Literal {
    fn accept<T: ExpressionVisitor>(&self, visitor: &mut T) -> T::Return {
        visitor.visit_literal(self)
    }
}

impl ExpressionNode for Unary {
    fn accept<T: ExpressionVisitor>(&self, visitor: &mut T) -> T::Return {
        visitor.visit_unary(self)
    }
}

impl ExpressionNode for Binary {
    fn accept<T: ExpressionVisitor>(&self, visitor: &mut T) -> T::Return {
        visitor.visit_binary(self)
    }
}

impl ExpressionNode for Identifier {
    fn accept<T: ExpressionVisitor>(&self, visitor: &mut T) -> T::Return {
        visitor.visit_identifier(self)
    }
}

impl ExpressionNode for Assignment {
    fn accept<T: ExpressionVisitor>(&self, visitor: &mut T) -> T::Return {
        visitor.visit_assignment(self)
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Minus => '-',
            Self::Not => '!',
        };
        write!(f, "{}", c)
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Addition => "+",
            Self::Division => "/",
            Self::Equality => "==",
            Self::Inequality => "!=",
            Self::StrictInferiority => "<",
            Self::StrictSuperiority => ">",
            Self::Superiority => ">=",
            Self::Inferiority => "<=",
            Self::Subtraction => "-",
            Self::Multiplication => "*",
            Self::Conjunction => "and",
            Self::Disjunction => "or",
        };
        write!(f, "{}", c)
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let expr = if self.priority() > self.expr.priority() {
            format!("({})", self.expr)
        } else {
            self.expr.to_string()
        };
        write!(f, "{}{}", self.op, expr)
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let left = if self.priority() > self.left.priority() {
            format!("({})", self.left)
        } else {
            self.left.to_string()
        };
        let right = if self.priority() >= self.right.priority() {
            format!("({})", self.right)
        } else {
            self.right.to_string()
        };

        write!(f, "{} {} {}", left, self.operator, right)
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match &self.value {
            LiteralValue::StringLiteral(s) => format!("\"{}\"", s),
            LiteralValue::NumberLiteral(f) => format!("{}", f),
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
        };
        write!(f, "{}", s)
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.ident.ident, self.expr)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(l) => write!(f, "{}", l),
            Self::BinaryOperation(b) => write!(f, "{}", b),
            Self::UnaryOperation(u) => write!(f, "{}", u),
            Self::Identifier(i) => write!(f, "{}", i.ident),
            Self::Assignment(a) => write!(f, "{}", a),
        }
    }
}

pub trait ExpressionNode {
    fn accept<T: ExpressionVisitor>(&self, visitor: &mut T) -> T::Return;
}

pub trait ExpressionVisitor: Sized {
    type Return;

    fn visit_expression(&mut self, expr: &Expression) -> Self::Return {
        match expr {
            Expression::Literal(l) => l.accept(self),
            Expression::UnaryOperation(u) => u.accept(self),
            Expression::BinaryOperation(b) => b.accept(self),
            Expression::Identifier(i) => i.accept(self),
            Expression::Assignment(a) => a.accept(self),
        }
    }

    fn visit_literal(&mut self, literal: &Literal) -> Self::Return;
    fn visit_unary(&mut self, unary: &Unary) -> Self::Return;
    fn visit_binary(&mut self, binary: &Binary) -> Self::Return;
    fn visit_identifier(&mut self, identifier: &Identifier) -> Self::Return;
    fn visit_assignment(&mut self, assignment: &Assignment) -> Self::Return;
}
