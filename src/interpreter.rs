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
        if self.current.is_some() { return self.current.clone() }
        if let Some(token) = self.tokenizer.next() {
            if token.kind != kind {
                panic!("Sintax error unexpected token {:?} at position {}",
                       token,
                       self.tokenizer.position
                       )
            }

            self.current = Some(token.clone());
            self.current.clone()
        } else {
            None
        }
    }

    pub fn consume_next(&mut self, kind: token::Kind) -> Option<token::Token> {
        if self.current.is_some() {
            let curr = self.current.clone();
            self.current = None;
            curr
        } else {
            let next = self.next(kind);
            self.current = None;
            next.clone()
        }
    }

    fn term(&mut self) -> token::Token {
        let mut result = self.consume_next(token::Kind::Integer).unwrap();

        if let Some(operator) = self.next(token::Kind::Operator) {
            result = match operator.value.as_ref() {
                "*" | "/" => {
                    self.consume_next(token::Kind::Operator);
                    if let Some(right) = self.consume_next(token::Kind::Integer) {
                        result.value = binary_operation(
                            result.clone().as_integer(),
                            operator.value,
                            right.as_integer()
                        ).to_string()
                    }
                    result
                },
                _ => result
            };
        }
        return result;
    }

    pub fn expr(&mut self) -> String {
        let mut result = self.term();
        while let Some(operator) = self.next(token::Kind::Operator) {
            self.consume_next(token::Kind::Operator);
            let right = self.term();

            let operation_result = match &*operator.value {
                "+" | "-" => binary_operation(
                                result.clone().as_integer(),
                                operator.value,
                                right.as_integer()
                            ),
                _ => result.clone().as_integer()
            };

            result.value = operation_result.to_string();
        }

        format!("{}", result.value)
    }
}

fn binary_operation(first: i32, operator: String, second: i32) -> i32 {
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

#[test]
fn it_respect_precedence() {
    let text = "1+1*2";
    let tokenizer = token::Tokenizer::new(String::from(text));
    let mut interpreter = Interpreter::new(tokenizer);

    assert_eq!("3", interpreter.expr());
}
