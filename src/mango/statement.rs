use super::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Expression { statement: Expression },
}
