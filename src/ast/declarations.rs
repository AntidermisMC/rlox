use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

use crate::ast::{
    expressions::{Expression, Identifier},
    types::Function,
    LiteralValue,
};

pub struct VariableDeclaration {
    pub name: Identifier,
    pub initializer: Expression,
}

impl Display for VariableDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "var {}{};",
            self.name.ident,
            match &self.initializer {
                Expression::Literal(l) if l.value == LiteralValue::Nil => "".to_string(),
                expr => format!(" = {}", expr),
            }
        )
    }
}

pub struct FunctionDeclaration {
    pub name: Identifier,
    pub function: Rc<Function>,
}

impl Display for FunctionDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "fun {}{}", self.name, self.function)
    }
}

pub struct ClassDeclaration {
    pub name: Identifier,
    pub methods: Vec<FunctionDeclaration>,
}

impl Display for ClassDeclaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "class {} {{", self.name)?;
        for method in &self.methods {
            writeln!(f, "{}{}", method.name, method.function)?;
        }
        write!(f, "}}")
    }
}
