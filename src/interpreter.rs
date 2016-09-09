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
        if let Some(token) = self.tokenizer.next() {
            if token.kind != kind {
                panic!("Sintax error unexpected token {:?} at position {}",
                       token,
                       self.tokenizer.position
                       )
            }

            self.current = Some(token.clone());
        }
        self.current.clone()
    }

    pub fn expr(&mut self) -> String {
        let mut operation_stack:Vec<token::Token> = vec![];

        while let Some(token) = self.tokenizer.next() {
            println!("{:?}", token);
            match token {
                token::Token{ kind: token::Kind::Integer, .. } =>
                    operation_stack.push(token),
                token::Token{ kind: token::Kind::Operator, .. } => {
                    let left = operation_stack.pop().unwrap();
                    let op = token;

                    if let Some(right) = self.next(token::Kind::Integer) {
                        let mut result = right.clone();

                        let res = eval_binary_operation(left.as_integer(),
                                                        op.value,
                                                        right.as_integer());
                        result.value = res.to_string();
                        operation_stack.push(result);
                    }
                },
                _ => break
            }
        }

        format!("{}", operation_stack.pop().unwrap().value)
    }
}

fn eval_binary_operation(first: i32, operator: String, second: i32) -> i32 {
    match &*operator {
        "+" => first + second,
        "-" => first - second,
        "*" => first * second,
        "/" => first / second,
        _ => panic!("Sintax error: invalid operator {}", operator)
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

#[test]
fn it_accepts_multiples_operation() {
    let text = "10+5-4-1";
    let tokenizer = token::Tokenizer::new(String::from(text));
    let mut interpreter = Interpreter::new(tokenizer);

    assert_eq!("10", interpreter.expr());
}
