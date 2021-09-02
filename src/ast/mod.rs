use std::fmt::{Display, Formatter};

pub enum Expression {
    Literal(Literal),
    UnaryOperation(Unary),
    BinaryOperation(Binary),
}

pub enum Literal {
    StringLiteral(String),
    NumberLiteral(f64),
    True,
    False,
    Nil,
}

pub struct Unary {
    pub op: UnaryOperator,
    pub expr: Box<Expression>,
}

pub enum UnaryOperator {
    Minus,
    Not,
}

pub struct Binary {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

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
}

// ########## Visitor stuff

pub trait AstNode {
    fn accept(&self, visitor: impl AstVisitor);
}

pub trait AstVisitor {
    fn visit_expr(&self, expr: &Expression);
    fn visit_literal(&self, literal: &Literal);
    fn visit_unary(&self, unary: &Unary);
    fn visit_binary(&self, binary: &Binary);
}

impl AstNode for Expression {
    fn accept(&self, visitor: impl AstVisitor) {
        visitor.visit_expr(self);
    }
}

impl AstNode for Literal {
    fn accept(&self, visitor: impl AstVisitor) {
        visitor.visit_literal(self);
    }
}

impl AstNode for Unary {
    fn accept(&self, visitor: impl AstVisitor) {
        visitor.visit_unary(self);
    }
}

impl AstNode for Binary {
    fn accept(&self, visitor: impl AstVisitor) {
        visitor.visit_binary(self);
    }
}

// ############## Display stuff

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
        };
        write!(f, "{}", c)
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.op, self.expr)
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::StringLiteral(s) => format!("\"{}\"", s),
            Self::NumberLiteral(f) => format!("{}", f),
            Self::Nil => "nil".to_string(),
            Self::True => "true".to_string(),
            Self::False => "false".to_string(),
        };
        write!(f, "{}", s)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(l) => write!(f, "{}", l),
            Self::BinaryOperation(b) => write!(f, "{}", b),
            Self::UnaryOperation(u) => write!(f, "{}", u),
        }
    }
}