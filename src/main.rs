mod mango;

use std::io::{self, Write};

use mango::interpreter::Interpreter;
use mango::parser::Parser;
use mango::scanner::Scanner;

fn main() {
    let mut interpreter = Interpreter::new();
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        // Assume a file path is passed
        let file_path = &args[1];
        match std::fs::read_to_string(file_path) {
            Ok(content) => {
                let tokens = Scanner::new(content).scan();
                let program = Parser::new(tokens).parse();
                interpreter.interpret(program);
            }
            Err(e) => {
                eprintln!("Error reading file {}: {}", file_path, e);
            }
        }
    } else {
        // Start the REPL
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
}
