use crate::eval::types::ValueType;
use std::collections::HashMap;

pub struct Environment {
    global: HashMap<String, ValueType>,
    stack: Vec<HashMap<String, ValueType>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            stack: Vec::new(),
            global: HashMap::new(),
        }
    }

    pub fn define(&mut self, identifier: String, value: ValueType) {
        if let Some(map) = self.stack.first_mut() {
            map.insert(identifier, value);
        } else {
            self.global.insert(identifier, value);
        }
    }

    pub fn get(&self, identifier: &str) -> Option<&ValueType> {
        for map in &self.stack {
            if let Some(value) = map.get(&identifier.to_string()) {
                // TODO why do I need .to_string() ?
                return Some(value);
            }
        }
        self.global.get(&identifier.to_string()) // TODO why do I need .to_string() ?
    }
}
