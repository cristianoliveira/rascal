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

    pub fn next(&mut self, expected_kind: token::Kind) -> Option<token::Token> {
        if self.current.is_some() { return self.current.clone() }
        if let Some(token) = self.tokenizer.next() {
            self.current = Some(token.clone());
            self.current.clone()
        } else {
            None
        }
    }

    pub fn consume_next(&mut self, expected_kind: token::Kind) -> Option<token::Token> {
        let token = match self.current {
            Some(_) => {
                let curr = self.current.clone();
                self.current = None;
                curr
            },
            None => {
                let next = self.next(expected_kind.clone());
                self.current = None;
                next.clone()
            }
        };

        if token.clone().unwrap().kind == expected_kind {
            return token;
        } else {
            panic!(
                "Sintax error: expected token kind {:?} found {:?} at position {}",
                expected_kind,
                token,
                self.tokenizer.position
                )
        }
    }

    fn term(&mut self) -> token::Token {
        let mut result = self.consume_next(token::Kind::Integer).unwrap();

        if let Some(operator) = self.next(token::Kind::Operator) {
            result = match operator.value.as_ref() {
                "*" | "/" => {
                    self.consume_next(token::Kind::Operator);
                    if let Some(right) = self.consume_next(token::Kind::Integer) {
                        result.value = 
                            binary_operation(&result, &operator, &right).to_string()
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

            let operation_result = match operator.value.as_ref() {
                "+" | "-" => binary_operation(&result, &operator, &right),
                _ => result.clone().as_integer()
            };

            result.value = operation_result.to_string();
        }

        format!("{}", result.value)
    }
}

fn binary_operation(first: &token::Token, operator: &token::Token, second: &token::Token) -> i32 {
    if first.kind == token::Kind::Integer {
        let operand = first.clone().as_integer();
        let operand2 = second.clone().as_integer();
        match &*operator.value {
            "+" => operand + operand2,
            "-" => operand - operand2,
            "*" => operand * operand2,
            "/" => operand / operand2,
            _ => panic!("Sintax error: invalid operator {}", operator.value)
        }
    } else {
        panic!("Sintax error: invalid binary operation using {} {} {}.",
               first.value,
               operator.value,
               second.value)
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
