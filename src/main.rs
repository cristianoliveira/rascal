mod token;
mod interpreter;

use std::io::{self,BufRead};

fn main() {
    let stdin = io::stdin();
    let line : String = stdin.lock().lines().next().unwrap().unwrap();
    //
    let tokenizer = token::Tokenizer::new(line);
    let mut iterpreter = interpreter::Interpreter::new(tokenizer);
    println!("result {}", iterpreter.expr())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
