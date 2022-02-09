use crate::ast::declarations::{FunctionDeclaration, VariableDeclaration};
use crate::ast::expressions::{Expression, ExpressionNode, ExpressionVisitor};
use crate::ast::statements::{Conditional, ForLoop, Statement, WhileLoop};
use crate::ast::types::ValueType;
use crate::eval::runtime_error::RuntimeError;
use crate::eval::Evaluator;
use crate::{eval, StatementVisitor};
use std::fmt::Write;

impl StatementVisitor for Evaluator {
    type Return = eval::Result<()>;

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
            Statement::Conditional(c) => self.visit_conditional(c),
            Statement::WhileLoop(w) => self.visit_while_loop(w),
            Statement::ForLoop(f) => self.visit_for_loop(f),
            Statement::FunctionDeclaration(f) => self.visit_function_declaration(f),
            Statement::Return(expr) => self.visit_return(expr),
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

    fn visit_conditional(&mut self, cond: &Conditional) -> Self::Return {
        let value = self.visit_expression(&cond.condition)?;
        if eval::is_truthy(&value.value) {
            Ok(self.visit_statement(&cond.then_statement)?)
        } else if let Some(else_statement) = &cond.else_statement {
            Ok(self.visit_statement(else_statement)?)
        } else {
            Ok(())
        }
    }

    fn visit_while_loop(&mut self, while_loop: &WhileLoop) -> Self::Return {
        while eval::is_truthy(&self.visit_expression(&while_loop.condition)?.value) {
            self.visit_statement(&while_loop.statement)?;
        }

        Ok(())
    }

    fn visit_for_loop(&mut self, for_loop: &ForLoop) -> Self::Return {
        self.env.push_env();
        if let Some(initializer) = &for_loop.initializer {
            self.visit_statement(initializer)?;
        }
        if let Some(condition) = &for_loop.condition {
            while eval::is_truthy(&self.visit_expression(condition)?.value) {
                self.visit_statement(&for_loop.body)?;
                if let Some(increment) = &for_loop.increment {
                    self.visit_expression(increment)?;
                }
            }
        } else {
            loop {
                self.visit_statement(&for_loop.body)?;
                if let Some(increment) = &for_loop.increment {
                    self.visit_expression(increment)?;
                }
            }
        }
        self.env.pop_env();

        Ok(())
    }

    fn visit_function_declaration(&mut self, fd: &FunctionDeclaration) -> Self::Return {
        self.env.define(
            fd.name.ident.to_string(),
            ValueType::Function(fd.function.clone()),
        );

        Ok(())
    }

    fn visit_return(&mut self, expr: &Expression) -> Self::Return {
        Err(RuntimeError::Return(self.visit_expression(expr)?))
    }
}
