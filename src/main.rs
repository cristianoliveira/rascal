mod token;
mod interpreter;

use std::io::{self,BufRead};

fn main() {
    let stdin = io::stdin();
    //
    while let Some(line) = stdin.lock().lines().next() {
        println!("{:?}", line);
        if let Ok(expr) = line {
            let tokenizer = token::Tokenizer::new(expr);
            let mut iterpreter = interpreter::Interpreter::new(tokenizer);
            println!("result {}", iterpreter.expr())
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
