use std::collections::HashMap;

use super::literal::Literal;

pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn access(&mut self, name: &String) -> &Literal {
        if let Some(value) = self.values.get(name) {
            return &value;
        } else {
            return &Literal::None;
        }
    }
}
