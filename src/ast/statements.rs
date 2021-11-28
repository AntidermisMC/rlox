use crate::ast::declarations::VariableDeclaration;
use crate::ast::expressions::Expression;
use std::fmt::{Display, Formatter};

pub enum Statement {
    Print(Expression),
    Expression(Expression),
    VariableDeclaration(VariableDeclaration),
    Block(Statements),
    Conditional(Box<Conditional>),
    WhileLoop(Box<WhileLoop>),
}

pub struct Statements {
    pub stmts: Vec<Statement>,
}

pub struct Conditional {
    pub condition: Expression,
    pub then_statement: Statement,
    pub else_statement: Option<Statement>,
}

pub struct WhileLoop {
    pub condition: Expression,
    pub statement: Statement,
}

impl Display for Statements {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for stmt in &self.stmts {
            write!(f, "{}\n", stmt)?
        }
        Ok(())
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Print(expr) => write!(f, "print {};", expr),
            Statement::Expression(expr) => write!(f, "{};", expr),
            Statement::VariableDeclaration(v) => write!(f, "{}", v),
            Statement::Block(stmts) => write!(f, "{{\n{}}}", stmts),
            Statement::Conditional(c) => write!(f, "{}", c),
            Statement::WhileLoop(l) => write!(f, "while ({}) {}", l.condition, l.statement),
        }
    }
}

impl Display for Conditional {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.else_statement {
            Some(else_statement) => write!(
                f,
                "if ({}) {} else {}",
                self.condition, self.then_statement, else_statement
            ),
            None => write!(f, "if ({}) {}", self.condition, self.then_statement),
        }
    }
}

pub trait StatementVisitor {
    type Return;

    fn visit_statement(&mut self, stmt: &Statement) -> Self::Return;
    fn visit_print(&mut self, expr: &Expression) -> Self::Return;
    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration) -> Self::Return;
    fn visit_conditional(&mut self, cond: &Conditional) -> Self::Return;
    fn visit_while_loop(&mut self, while_loop: &WhileLoop) -> Self::Return;
}
