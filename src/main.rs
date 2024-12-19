mod mango;

use mango::interpreter::Interpreter;
use mango::parser::Parser;
use mango::scanner::Scanner;

fn main() {
    let code = "var a = 1; var b = 2; print a + b;";

    let mut interpreter = Interpreter::new();

    let tokens = Scanner::new(code.to_string()).scan();
    let expression = Parser::new(tokens).parse();

    interpreter.interpret(expression);
}
