use token;
use ast;

// # Interpreter
//
// Represents the interpreter that is responsible for interpret 
// the stream of token from a given Tokenizer
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

    // next
    //
    // It store the next token from Tokenizer and return itself for
    // chaining porpouses
    fn next(&mut self) -> &mut Self {
        if self.current.is_none() { self.current = self.tokenizer.next() }
        self
    }

    // get
    //
    // It gets the current token without consuming it
    fn get(&mut self) -> Option<token::Token> {
        self.current.clone()
    }

    // consume
    //
    // It is responsible for consume the current Token
    fn consume(&mut self, expected_kind: token::Kind) -> token::Token {
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
            return token;
        } else {
            panic!("Interpreter error: unexpected end of file");
        }
    }

    // factor
    //
    // factor can be a terminal Integer or result of a grouped expr
    // represented as context free grammar:
    // ```
    //  factor:: Integer
    //  factor:: ( expr )
    // ```
    fn factor(&mut self) -> ast::Node {
        match self.next().get() {
            Some(token::Token{ kind: token::Kind::BlockBegin , .. }) => {
                self.consume(token::Kind::BlockBegin);
                let result = self.expr();
                self.consume(token::Kind::BlockEnd);
                result
            },
            Some(token::Token{ kind: token::Kind::Integer, .. }) => {
                ast::Node::leaf(self.next().consume(token::Kind::Integer))
            },
            _ => panic!("Error factor")
        }
    }

    // term
    //
    // One term can be a `factor` or result of `factor * factor`
    // or `factor / factor represented in context free grammar:
    // ```
    // term:: factor
    // term:: factor (*|/) factor
    // ```
    fn term(&mut self) -> ast::Node {
        let mut result = self.factor();

        if let Some(token) = self.next().get() {
            match token.value.as_ref() {
                "*" | "/" => {
                    let operator = self.consume(token::Kind::Operator);
                    let right = self.factor();
                    return ast::Node::new(Some(result), operator, Some(right));
                },
                _ => return result
            };
        }
        return result
    }

    // # expr
    //
    // One expr can be a `term` or result of `term + term` or `term - term`
    // represented in context free grammar:
    // ```
    //   expr:: term
    //   expr:: term (+|-) term
    // ```
    pub fn expr(&mut self) -> ast::Node {
        let mut result = self.term();
        while let Some(token) = self.next().get() {
            if token.kind == token::Kind::EOF { break }
            match token.value.as_ref() {
                "+" | "-" => {
                    let operator = self.consume(token::Kind::Operator);
                    let right = self.term();
                    result = ast::Node::new(Some(result.clone()), operator, Some(right))
                },
                _ => break
            };
        }
        result
    }

    pub fn eval_tree(&mut self) -> String {
        let tree = self.expr();
        ast::eval_tree(tree)
    }
}

#[test]
fn it_sums() {
    let text = "5+1";
    let tokenizer = token::Tokenizer::new(String::from(text));
    let mut interpreter = Interpreter::new(tokenizer);

    assert_eq!("6", interpreter.eval_tree());
}

#[test]
fn it_substract() {
    let text = "5-1";
    let tokenizer = token::Tokenizer::new(String::from(text));
    let mut interpreter = Interpreter::new(tokenizer);

    assert_eq!("4", interpreter.eval_tree());
}

#[test]
fn it_multiplies() {
    let text = "5*2";
    let tokenizer = token::Tokenizer::new(String::from(text));
    let mut interpreter = Interpreter::new(tokenizer);

    assert_eq!("10", interpreter.eval_tree());
}

#[test]
fn it_divide() {
    let text = "4/2";
    let tokenizer = token::Tokenizer::new(String::from(text));
    let mut interpreter = Interpreter::new(tokenizer);

    assert_eq!("2", interpreter.eval_tree());
}

#[test]
fn it_accepts_multiples_operation() {
    let text = "10+5-4-1";
    let tokenizer = token::Tokenizer::new(String::from(text));
    let mut interpreter = Interpreter::new(tokenizer);

    assert_eq!("10", interpreter.eval_tree());
}

#[test]
fn it_respect_precedence() {
    let text = "1+1*2";
    let tokenizer = token::Tokenizer::new(String::from(text));
    let mut interpreter = Interpreter::new(tokenizer);

    assert_eq!("3", interpreter.eval_tree());
}

#[test]
fn it_respects_grouped_expression() {
    let text = "4+(1+(1+1)*2)+1";
    let tokenizer = token::Tokenizer::new(String::from(text));
    let mut interpreter = Interpreter::new(tokenizer);

    assert_eq!("10", interpreter.eval_tree());
}
