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

    pub fn next(&mut self) -> &mut Self {
        if self.current.is_none() { self.current = self.tokenizer.next() }
        self
    }

    pub fn get(&mut self) -> Option<token::Token> {
        self.current.clone()
    }

    pub fn consume(&mut self, expected_kind: token::Kind) -> Option<token::Token> {
        let mut cosumed = self.current.clone();
        if let Some(token) = self.current.clone() {
            self.current = None;
            if token.kind != expected_kind {
                panic!(
                    "Sintax error: expected token kind {:?} found {:?} at position {}",
                    expected_kind,
                    token,
                    self.tokenizer.position
                    )
            }
        }
        return cosumed;
    }

    fn factor(&mut self) -> token::Token {
        match self.next().get() {
            Some(token::Token{ kind: token::Kind::BlockBegin , .. }) => {
                self.consume(token::Kind::BlockBegin);
                return token::Token::build(token::Kind::Integer, self.expr());
            },
            Some(token::Token{ kind: token::Kind::Integer, .. }) => {
                return self.next().consume(token::Kind::Integer).unwrap();
            },
            _ => panic!("Error factor")
        }
    }

    fn term(&mut self) -> token::Token {
        let mut result = self.factor();

        if let Some(operator) = self.next().get() {
            result.value = match operator.value.as_ref() {
                "*" | "/" => {
                    self.consume(token::Kind::Operator);
                    let right = self.factor();
                    binary_operation(
                        &result,
                        &operator,
                        &right
                    ).to_string()
                },
                _ => result.clone().value
            };
        }
        return result;
    }

    pub fn expr(&mut self) -> String {
        let mut result = self.term();
        while let Some(operator) = self.next().get() {
            if operator.kind == token::Kind::BlockEnd {
                self.consume(token::Kind::BlockEnd);
                break;
            }
            self.consume(token::Kind::Operator);
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

#[test]
fn it_respects_grouped_expression() {
    let text = "4+(1+(1+1)*2)+1";
    let tokenizer = token::Tokenizer::new(String::from(text));
    let mut interpreter = Interpreter::new(tokenizer);

    assert_eq!("10", interpreter.expr());
}
