use super::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Program { statements: Vec<Statement> },
    Block { statements: Vec<Statement> },

    Expression { expression: Expression },
    VariableDeclaration { name: String, value: Expression },
    While { condition: Expression, block: Box<Statement> },
    
    Print { expression: Expression },
}

impl Statement {
    pub fn accept<T, V: Visitor<T>>(&self, visitor: &mut V) -> T {
        match self {
            Statement::Program { statements } => visitor.visit_program(statements),
            Statement::Block { statements } => visitor.visit_block(statements),

            Statement::Expression { expression } => visitor.visit_expression(expression),
            Statement::VariableDeclaration { name, value } => {
                visitor.visit_variable_declaration(name, value)
            },
            Statement::While { condition, block  } => visitor.visit_while(condition, block),

            Statement::Print { expression } => visitor.visit_print(expression),
        }
    }
}

pub trait Visitor<T> {
    fn visit_program(&mut self, statements: &Vec<Statement>) -> T;
    fn visit_block(&mut self, statements: &Vec<Statement>) -> T;

    fn visit_expression(&mut self, expression: &Expression) -> T;
    fn visit_variable_declaration(&mut self, name: &String, value: &Expression) -> T;
    fn visit_while(&mut self, condition: &Expression, block: &Box<Statement>) -> T;

    fn visit_print(&mut self, expression: &Expression) -> T;
}
