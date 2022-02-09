use crate::ast::declarations::{FunctionDeclaration, VariableDeclaration};
use crate::ast::expressions::Expression;
use crate::ast::LiteralValue;
use std::fmt::{Debug, Display, Formatter};

pub enum Statement {
    Print(Expression),
    Expression(Expression),
    VariableDeclaration(VariableDeclaration),
    Block(Statements),
    Conditional(Box<Conditional>),
    WhileLoop(Box<WhileLoop>),
    ForLoop(Box<ForLoop>),
    FunctionDeclaration(FunctionDeclaration),
    Return(Expression),
}

pub struct Statements {
    pub stmts: Vec<Statement>,
}

impl Debug for Statements {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for stmt in &self.stmts {
            writeln!(f, "{}", stmt)?
        }
        Ok(())
    }
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

pub struct ForLoop {
    pub initializer: Option<Statement>,
    pub condition: Option<Expression>,
    pub increment: Option<Expression>,
    pub body: Statement,
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
            Statement::ForLoop(l) => write!(f, "{}", l),
            Statement::FunctionDeclaration(fd) => write!(f, "{}", fd),
            Statement::Return(expr) => match expr {
                Expression::Literal(l) if l.value == LiteralValue::Nil => {
                    write!(f, "return;")
                }
                _ => write!(f, "return {};", expr),
            },
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

impl Display for ForLoop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "for (")?;
        if let Some(init) = &self.initializer {
            write!(f, "{}", init)?;
        } else {
            write!(f, ";")?;
        }
        if let Some(cond) = &self.condition {
            write!(f, " {}", cond)?;
        }
        write!(f, ";")?;
        if let Some(increment) = &self.increment {
            write!(f, " {}", increment)?;
        }
        write!(f, ") {}", self.body)
    }
}

pub trait StatementVisitor {
    type Return;

    fn visit_statement(&mut self, stmt: &Statement) -> Self::Return;
    fn visit_print(&mut self, expr: &Expression) -> Self::Return;
    fn visit_variable_declaration(&mut self, decl: &VariableDeclaration) -> Self::Return;
    fn visit_conditional(&mut self, cond: &Conditional) -> Self::Return;
    fn visit_while_loop(&mut self, while_loop: &WhileLoop) -> Self::Return;
    fn visit_for_loop(&mut self, for_loop: &ForLoop) -> Self::Return;
    fn visit_function_declaration(&mut self, fd: &FunctionDeclaration) -> Self::Return;
}
