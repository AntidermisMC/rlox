mod conversions;

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

// ########## Priorities

pub trait Priority {
    fn priority(&self) -> u8;
}

impl Priority for BinaryOperator {
    fn priority(&self) -> u8 {
        match self {
            BinaryOperator::Equality => 0,
            BinaryOperator::Inequality => 0,
            BinaryOperator::StrictInferiority => 1,
            BinaryOperator::Inferiority => 1,
            BinaryOperator::StrictSuperiority => 1,
            BinaryOperator::Superiority => 1,
            BinaryOperator::Addition => 2,
            BinaryOperator::Subtraction => 2,
            BinaryOperator::Multiplication => 3,
            BinaryOperator::Division => 3,
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
        4
    }
}

impl Priority for Unary {
    fn priority(&self) -> u8 {
        self.op.priority()
    }
}

impl Priority for Literal {
    fn priority(&self) -> u8 {
        5
    }
}

impl Priority for Expression {
    fn priority(&self) -> u8 {
        match self {
            Expression::Literal(l) => l.priority(),
            Expression::UnaryOperation(u) => u.priority(),
            Expression::BinaryOperation(b) => b.operator.priority(),
        }
    }
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

impl AstNode for Expression {
    fn accept<T: AstVisitor>(&self, visitor: &T) -> T::Return {
        visitor.visit_expr(self)
    }
}

impl AstNode for Literal {
    fn accept<T: AstVisitor>(&self, visitor: &T) -> T::Return {
        visitor.visit_literal(self)
    }
}

impl AstNode for Unary {
    fn accept<T: AstVisitor>(&self, visitor: &T) -> T::Return {
        visitor.visit_unary(self)
    }
}

impl AstNode for Binary {
    fn accept<T: AstVisitor>(&self, visitor: &T) -> T::Return {
        visitor.visit_binary(self)
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
