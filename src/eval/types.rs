use crate::code_span::CodeSpan;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub enum ValueType {
    String(Rc<String>),
    Number(f64),
    Boolean(bool),
    Nil,
    Object(Object),
}

#[derive(PartialEq, Debug)]
pub struct Value {
    pub location: CodeSpan,
    pub value: ValueType,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Type {
    String,
    Number,
    Boolean,
    Nil,
    Object,
}

pub type Object = Rc<HashMap<String, Value>>;

impl ValueType {
    pub fn as_type(&self) -> Type {
        match self {
            ValueType::String(_) => Type::String,
            ValueType::Number(_) => Type::Number,
            ValueType::Boolean(_) => Type::Boolean,
            ValueType::Nil => Type::Nil,
            ValueType::Object(_) => Type::Object,
        }
    }
}

impl Value {
    pub fn new(value: ValueType, location: CodeSpan) -> Self {
        Self { value, location }
    }
}

impl From<&ValueType> for Type {
    fn from(value: &ValueType) -> Self {
        match value {
            ValueType::String(_) => Type::String,
            ValueType::Number(_) => Type::Number,
            ValueType::Boolean(_) => Type::Boolean,
            ValueType::Nil => Type::Nil,
            ValueType::Object(_) => Type::Object,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::String(s) => write!(f, "{}", s),
            ValueType::Number(n) => write!(f, "{}", n),
            ValueType::Boolean(b) => write!(f, "{}", b),
            ValueType::Nil => write!(f, "nil"),
            ValueType::Object(_) => write!(f, "[Object object]"),
        }
    }
}
