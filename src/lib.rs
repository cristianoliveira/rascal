mod token;
mod interpreter;
mod ast;
mod parser;
mod primitive;
mod frame;
pub mod repl;

pub fn eval(source: String) -> String {
    let tokenizer = token::Tokenizer::new(source);
    let mut parser = parser::Parser::new(tokenizer);
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.eval(parser.parse())
}
