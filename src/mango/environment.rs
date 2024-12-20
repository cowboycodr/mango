use std::collections::HashMap;

use super::literal::Literal;

#[derive(Clone, Debug)]
pub struct Environment {
    values: HashMap<String, Literal>,
    pub enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn from_enclosing(enclosing: Environment) -> Self {
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

    pub fn assign(&mut self, name: &String, value: Literal) -> Literal {
        if self.values.contains_key(name) {
            self.values.insert(name.clone(), value.clone());
            value
        } else if let Some(ref mut enclosing) = self.enclosing {
            enclosing.assign(name, value)
        } else {
            panic!("Undefined variable '{}'.", name);
        }
    }
}
