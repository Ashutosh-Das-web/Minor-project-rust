mod lexer;
mod parser;
mod executor;
mod builtin;
mod ast;

use std::io::{self, Write};

fn main() {
    repl_loop();
}

/// Main shell loop
fn repl_loop() {
    loop {
        print_prompt();

        let input = read_input();

        // Ignore empty input
        if input.is_empty() {
            continue;
        }

        // Built-in exit
        if input == "exit" {
            println!("Bye!");
            break;
        }
        // STEP 1 : Tokenize
        let tokens = lexer::tokenize(&input);
        println!("Tokens : {:?}", tokens);
        // STEP 2 : Parse
        let pipeline = parser::parse(tokens);
        println!("Pipeline Parsed!");
        // STEP 3 : Execute
        executor::execute(pipeline);
    }
}
/// Print shell prompt
fn print_prompt() {
    print!("minish> ");
    io::stdout().flush().unwrap();
}
/// Read one line from stdin
fn read_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input");

    input.trim().to_string()
}