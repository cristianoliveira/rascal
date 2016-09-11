mod token;
mod interpreter;
mod ast;
mod parser;

use std::io::{self,BufRead};

fn main() {
    let stdin = io::stdin();
    while let Some(line) = stdin.lock().lines().next() {
        print!(">>");
        if let Ok(expr) = line {
            let tokenizer = token::Tokenizer::new(expr);
            let parser = parser::Parser::new(tokenizer);
            let mut iterpreter = interpreter::Interpreter::new(parser);
            println!(">> {}", iterpreter.eval_tree());
            println!(">>");
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
