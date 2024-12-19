mod mango;
mod utils;

use std::io::{self, Write};

use mango::interpreter::Interpreter;
use mango::parser::Parser;
use mango::scanner::Scanner;

use utils::timer::Timer;

fn main() {
    let interpreter = Interpreter::new();
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let file_path = &args[1];
        run_file(file_path, interpreter);
    } else {
        repl(interpreter);
    }
}

fn run_file(file_path: &String, mut interpreter: Interpreter) {
    let timer = Timer::start();

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

    println!("Completed in {} milliseconds.", timer.end().as_millis());
}

fn repl(mut interpreter: Interpreter) {
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

        let timer = Timer::start();

        let tokens = Scanner::new(input.to_string()).scan();
        let program = Parser::new(tokens).parse();

        interpreter.interpret(program);

        println!("Completed in {} milliseconds.", timer.end().as_millis());
    }
}