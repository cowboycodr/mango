use std::collections::HashMap;

use super::literal::Literal;

pub struct Environment {
    values: HashMap<String, Literal>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn from_environment(enclosing: Environment) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn access(&mut self, name: &String) -> Option<&Literal> {
        if let Some(value) = self.values.get(name) {
            return Some(value);
        } else if let Some(enclosing) = &mut self.enclosing {
            return enclosing.access(name);
        }

        None
    }
}
