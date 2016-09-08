use token;

pub struct Interpreter {
    tokenizer: token::Tokenizer,
    current: Option<token::Token>
}

impl Interpreter {
    pub fn new(text: token::Tokenizer) -> Self {
        Interpreter {
            tokenizer: text,
            current: None
        }
    }

    pub fn next(&mut self, kind: token::Kind) -> Option<token::Token> {
        let current = self.tokenizer.next();
        if let Some(token) = current {
            self.current = Some(token);
            self.current.clone()
        } else {
            panic!("Sintax error")
        }
    }

    pub fn expr(&mut self) -> String {
        let left = self.next(token::Kind::Integer);
        let op = self.next(token::Kind::Operator);
        let right = self.next(token::Kind::Integer);

        format!("{}", calc(left.unwrap(), op.unwrap(), right.unwrap()))
    }
}

fn calc(first: token::Token, operator: token::Token, second: token::Token) -> i32 {
    println!("{:?}", operator);
    match &*operator.value {
        "+" => first.as_integer() + second.as_integer(),
        "-" => first.as_integer() - second.as_integer(),
        "*" => first.as_integer() * second.as_integer(),
        "/" => first.as_integer() / second.as_integer(),
        _ => panic!("Sintax error")
    }
}

#[test]
fn it_sums() {
    let text = "5+1";
    let tokenizer = token::Tokenizer::new(String::from(text));
    let mut interpreter = Interpreter::new(tokenizer);

    assert_eq!("6", interpreter.expr());
}

#[test]
fn it_substract() {
    let text = "5-1";
    let tokenizer = token::Tokenizer::new(String::from(text));
    let mut interpreter = Interpreter::new(tokenizer);

    assert_eq!("4", interpreter.expr());
}

#[test]
fn it_multiplies() {
    let text = "5*2";
    let tokenizer = token::Tokenizer::new(String::from(text));
    let mut interpreter = Interpreter::new(tokenizer);

    assert_eq!("10", interpreter.expr());
}

#[test]
fn it_divide() {
    let text = "4/2";
    let tokenizer = token::Tokenizer::new(String::from(text));
    let mut interpreter = Interpreter::new(tokenizer);

    assert_eq!("2", interpreter.expr());
}
