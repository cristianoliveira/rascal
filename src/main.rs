mod token;
mod interpreter;
mod ast;

use std::io::{self,BufRead};

fn main() {
    let stdin = io::stdin();
    while let Some(line) = stdin.lock().lines().next() {
        print!(">>");
        if let Ok(expr) = line {
            let tokenizer = token::Tokenizer::new(expr);
            let mut iterpreter = interpreter::Interpreter::new(tokenizer);
            println!(">> {}", iterpreter.expr());
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
