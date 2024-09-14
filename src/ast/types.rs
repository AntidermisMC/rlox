use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
    rc::Rc,
};

use crate::{
    ast::{expressions::Identifier, statements::Statements},
    code_span::CodeSpan,
    eval::Result,
};

#[derive(Clone, Debug)]
pub enum ValueType {
    String(Rc<String>),
    Number(f64),
    Boolean(bool),
    Nil,
    Object(Rc<Object>),
    NativeFunction(NativeFunction, usize),
    Function(Rc<Function>),
    Class(Rc<Class>),
}

#[derive(PartialEq, Clone, Debug)]
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
    NativeFunction,
    Function,
    Class,
}

#[derive(Clone)]
pub struct Object {
    pub properties: HashMap<String, Value>,
    pub class: Rc<Class>,

}

pub type NativeFunction = fn(Vec<ValueType>, CodeSpan) -> Result<ValueType>;

#[derive(Debug)]
pub struct Function {
    pub args: Vec<Identifier>,
    pub body: Statements,
    pub span: CodeSpan,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: Identifier,
}

impl ValueType {
    pub fn as_type(&self) -> Type {
        match self {
            ValueType::String(_) => Type::String,
            ValueType::Number(_) => Type::Number,
            ValueType::Boolean(_) => Type::Boolean,
            ValueType::Nil => Type::Nil,
            ValueType::Object(_) => Type::Object,
            ValueType::NativeFunction(_, _) => Type::NativeFunction,
            ValueType::Function(_) => Type::Function,
            ValueType::Class(_) => Type::Class,
        }
    }
}

impl Value {
    pub fn new(value: ValueType, location: CodeSpan) -> Self {
        Self { value, location }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{", self.class.name)?;
        f.write_str("{")?;
        let mut iter = self.properties.iter();
        if let Some((name, value)) = iter.next() {
            write!(f, "{}: {}", name, value)?
        }
        for (name, value) in iter {
            write!(f, ",\n{}: {}", name, value)?
        }
        f.write_str("}")?;
        Ok(())
    }
}

impl From<&ValueType> for Type {
    fn from(value: &ValueType) -> Self {
        value.as_type()
    }
}

impl PartialEq for ValueType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ValueType::String(s1), ValueType::String(s2)) => s1 == s2,
            (ValueType::Nil, ValueType::Nil) => true,
            (ValueType::Object(_), ValueType::Object(_)) => todo!(),
            (ValueType::Boolean(b1), ValueType::Boolean(b2)) => b1 == b2,
            (ValueType::NativeFunction(f1, _), ValueType::NativeFunction(f2, _)) => f1 == f2,
            (ValueType::Number(n1), ValueType::Number(n2)) => n1 == n2,
            (ValueType::Function(f1), ValueType::Function(f2)) => Rc::ptr_eq(f1, f2),
            (_, _) => false,
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
            ValueType::Object(_) => write!(f, "{}", todo!()),
            ValueType::NativeFunction(_, _) => write!(f, "<native fn>"),
            ValueType::Function(_) => write!(f, "<function>"),
            ValueType::Class(class) => write!(f, "{}", class.name),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        let mut iter = self.args.iter();

        if let Some(first_arg) = iter.next() {
            write!(f, "{}", first_arg)?;
            for arg in iter {
                write!(f, ", {}", arg)?;
            }
        }

        write!(f, ") {{ {} }}", self.body)
    }
}
