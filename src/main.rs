extern crate rustc_serialize;
extern crate docopt;

mod token;
mod interpreter;
mod ast;
mod parser;

use std::io::{self};
use std::io::prelude::*;
use std::fs::File;

use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Rascal.

Usage:
  rascal <source>
  rascal (-r)
  rascal (-h | --help)
  rascal (-v | --version)

Options:
  -r --repl         Opens the REPL.
  -h --help         Shows this message.
  -v --version      Shows version.
  --verbose         Use verbose output.
";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub arg_source: Vec<String>,
    pub flag_r: bool,
    pub flag_h: bool,
    pub flag_v: bool,

}

// REPL
// Interpret the expression for a given std input.
// accepts a file to interpret
fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.decode()) .unwrap_or_else(|e| e.exit());

    match args {
        Args { flag_v: true, ..} => println!("{}", VERSION),
        Args { flag_r: true, ..} => {
            let stdin = io::stdin();
            while let Some(line) = stdin.lock().lines().next() {
                if let Ok(expr) = line {
                    let tokenizer = token::Tokenizer::new(expr);
                    let mut parser = parser::Parser::new(tokenizer);
                    let mut interpreter = interpreter::Interpreter::new();
                    let result = interpreter.eval(parser.parse());
                    println!(">> {}", result);
                }
            }
        },
        Args { ref arg_source, ..} => {
            println!("{:?}", arg_source);
            let mut f = File::open(&arg_source[0]).unwrap();
            let mut source_code = String::new();
            let _ = f.read_to_string(&mut source_code);
            let tokenizer = token::Tokenizer::new(source_code);
            let mut parser = parser::Parser::new(tokenizer);
            let mut interpreter = interpreter::Interpreter::new();
            let result = interpreter.eval(parser.parse());
            println!(">> {}", result);
        }
    }
}
