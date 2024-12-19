use super::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Program { statements: Vec<Statement> },

    Print { expression: Expression },
    Expression { expression: Expression },
    VariableDeclaration { name: String, value: Expression },
}

impl Statement {
    pub fn accept<T, V: Visitor<T>>(&self, visitor: &mut V) -> T {
        match self {
            Statement::Program { statements } => visitor.visit_program(statements),

            Statement::Print { expression } => visitor.visit_print(expression),
            Statement::Expression { expression } => visitor.visit_expression(expression),
            Statement::VariableDeclaration { name, value } => {
                visitor.visit_variable_declaration(name, value)
            }
        }
    }
}

pub trait Visitor<T> {
    fn visit_program(&mut self, statements: &Vec<Statement>) -> T;

    fn visit_print(&mut self, expression: &Expression) -> T;
    fn visit_expression(&mut self, expression: &Expression) -> T;
    fn visit_variable_declaration(&mut self, name: &String, value: &Expression) -> T;
}
