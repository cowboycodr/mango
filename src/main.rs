mod mango;

use std::io::{self, Write};

use mango::interpreter::Interpreter;
use mango::parser::Parser;
use mango::scanner::Scanner;

fn main() {
    let mut interpreter = Interpreter::new();

    loop {
        print!("> "); // Print the prompt
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("exit") {
            println!("Exiting the loop. Goodbye!");
            break;
        }

        let tokens = Scanner::new(input.to_string()).scan();
        let program = Parser::new(tokens).parse();

        interpreter.interpret(program);
    }
}
