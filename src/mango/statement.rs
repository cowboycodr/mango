use super::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Expression { expression: Expression },
    Print { expression: Expression },
}

impl Statement {
    pub fn accept<T, V: Visitor<T>>(&self, visitor: &mut V) -> T {
        match self {
            Statement::Expression { expression } => visitor.visit_expression(expression),
            Statement::Print { expression } => visitor.visit_print(expression),
        }
    }
}

pub trait Visitor<T> {
    fn visit_expression(&mut self, expression: &Expression) -> T;
    fn visit_print(&mut self, expression: &Expression) -> T;
}
