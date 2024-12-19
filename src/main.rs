// mod mango;

// use std::io::{self, Write};

// use mango::parser::Parser;
// use mango::scanner::Scanner;

// fn main() {
//     loop {
//         let mut input = String::new();

//         print!("> ");
//         io::stdout().flush().expect("Failed to flush stdout"); // Flush to ensure prompt is shown

//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read input");

//         let trimmed = input.trim();

//         if trimmed.eq_ignore_ascii_case("exit") {
//             break;
//         }

//         let mut scanner = Scanner::new(input.clone());
//         let tokens = scanner.scan();

//         let mut parser = Parser::new(tokens);
//         let expression = parser.parse();

//         println!("{}", expression.evaluate());
//     }
// }

mod mango;

use mango::interpreter::Interpreter;
use mango::scanner::Scanner;
use mango::parser::Parser;

fn main() {
    let code = "2 ** 2";

    let mut interpreter = Interpreter::new();

    let tokens = Scanner::new(code.to_string()).scan();
    let expression = Parser::new(tokens).parse();

    println!("{}", interpreter.interpret(expression));
}