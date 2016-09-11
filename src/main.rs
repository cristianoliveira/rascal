mod token;
mod interpreter;
mod ast;
mod parser;

use std::io::{self,BufRead};

// REPL
// Interpret the expression for a given std input.
fn main() {
    let stdin = io::stdin();
    while let Some(line) = stdin.lock().lines().next() {
        if let Ok(expr) = line {
            let tokenizer = token::Tokenizer::new(expr);
            let mut parser = parser::Parser::new(tokenizer);
            let mut interpreter = interpreter::Interpreter::new();
            let result = interpreter.eval_tree(parser.parse());
            println!(">> {}", result);
        }
    }
}
