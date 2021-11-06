use crate::ast::expressions::Expression;
use std::fmt::{Display, Formatter};
use std::vec::IntoIter;

pub enum Statement {
    Print(Expression),
    Expression(Expression),
}

pub struct Statements {
    pub stmts: Vec<Statement>,
}

impl Display for Statements {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for stmt in &self.stmts {
            write!(f, "{}\n", stmt)?
        }
        Ok(())
    }
}

impl IntoIterator for Statements {
    type Item = Statement;
    type IntoIter = IntoIter<Statement>;

    fn into_iter(self) -> Self::IntoIter {
        self.stmts.into_iter()
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Print(expr) => write!(f, "print {}", expr),
            Statement::Expression(expr) => write!(f, "{}", expr),
        }
    }
}
