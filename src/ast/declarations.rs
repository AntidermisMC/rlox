use crate::ast::expressions::{Expression, Identifier};
use crate::ast::LiteralValue;
use std::fmt::{Display, Formatter};

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
