mod mango;

use mango::interpreter::Interpreter;
use mango::parser::Parser;
use mango::scanner::Scanner;

fn main() {
    let code = "2 ** 2";

    let mut interpreter = Interpreter::new();

    let tokens = Scanner::new(code.to_string()).scan();
    let expression = Parser::new(tokens).parse();

    println!("{}", interpreter.interpret(expression));
}
