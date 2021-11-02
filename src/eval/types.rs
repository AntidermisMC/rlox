use std::collections::HashMap;
use crate::code_span::CodeSpan;

#[derive(PartialEq, Debug)]
pub enum ValueType {
    String(String),
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

pub type Object = HashMap<String, Value>;

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