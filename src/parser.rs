use token::{Token, Kind, Tokenizer};
use ast;

// # Parser
//
// Represents the parser that is responsible for parsing a stream of tokens
// from a given Tokenizer into an Abstracted Sintax Tree
pub struct Parser {
    tokenizer: Tokenizer,
    current: Option<Token>
}

impl Parser {
    pub fn new(text: Tokenizer) -> Self {
        Parser {
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
    fn get(&mut self) -> Option<Token> {
        self.current.clone()
    }

    // consume
    //
    // It is responsible for consume the current Token validating the expected
    // token for the expression sintax
    fn consume(&mut self, expected_kind: Kind) -> Token {
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
    // factor can be a terminal Integer, result of a grouped expr
    // or unary result of a factor. Represented as context free grammar:
    // ```
    //  factor:: (-|+) factor
    //  factor:: Integer
    //  factor:: ( expr )
    // ```
    fn factor(&mut self) -> ast::Node {
        match self.next().get() {
            Some(Token{ kind: Kind::Operator, .. }) => {
                let token = self.consume(Kind::Operator);
                ast::Node::unary(token, self.factor())
            },
            Some(Token{ kind: Kind::GroupBegin , .. }) => {
                self.consume(Kind::GroupBegin);
                let result = self.expr();
                self.consume(Kind::GroupEnd);
                result
            },
            Some(Token{ kind: Kind::Integer, .. }) => {
                ast::Node::leaf(self.next().consume(Kind::Integer))
            },
            _ => panic!("Error factor")
        }
    }

    // term
    //
    // One term can be a `factor` or result of `factor * factor`
    // or `factor / factor`. Represented in context free grammar:
    // ```
    // term:: factor
    // term:: factor (*|/) factor
    // ```
    fn term(&mut self) -> ast::Node {
        let mut result = self.factor();

        if let Some(token) = self.next().get() {
            match token.value.as_ref() {
                "*" | "/" => {
                    let operator = self.consume(Kind::Operator);
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
    // Represented in context free grammar:
    // ```
    //   expr:: term
    //   expr:: term (+|-) term
    // ```
    pub fn expr(&mut self) -> ast::Node {
        let mut result = self.term();
        while let Some(token) = self.next().get() {
            if token.kind == Kind::EOF { break }
            match token.value.as_ref() {
                "+" | "-" => {
                    let operator = self.consume(Kind::Operator);
                    let right = self.term();
                    result = ast::Node::new(Some(result.clone()), operator, Some(right))
                },
                _ => break
            };
        }
        result
    }

    pub fn parse(&mut self) -> ast::Node {
        self.expr()
    }
}


fn test_node_builder(left: String, operator: String, right: String) -> ast::Node {
    let lnode = ast::Node::leaf(Token::build(Kind::Integer, left));
    let token = Token::build(Kind::Operator, operator);
    let rnode = ast::Node::leaf(Token::build(Kind::Integer, right));
    ast::Node::new(Some(lnode), token, Some(rnode))
}

#[test]
fn it_parses_sum_as_node() {
    let text = "5+1";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);
    let expected = test_node_builder(String::from("5"),
                                     String::from("+"),
                                     String::from("1"));

    assert_eq!(expected, parser.parse());
}

#[test]
fn it_parses_multiples_operation() {
    let text = "10+5-4";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    let firstsum = test_node_builder(String::from("10"),
                                     String::from("+"),
                                     String::from("5"));

    let token = Token::build(Kind::Operator, String::from("-"));
    let rnode = ast::Node::leaf(Token::build(Kind::Integer, String::from("4")));

    let expected = ast::Node::new(Some(firstsum), token, Some(rnode));
    assert_eq!(expected, parser.parse());
}

#[test]
fn it_parses_respecting_precedence() {
    let text = "10+5*4";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    let plusnode = test_node_builder(String::from("5"),
                                     String::from("*"),
                                     String::from("4"));

    let token = Token::build(Kind::Operator, String::from("+"));
    let rnode = ast::Node::leaf(Token::build(Kind::Integer, String::from("10")));

    let expected = ast::Node::new(Some(rnode), token, Some(plusnode));
    assert_eq!(expected, parser.parse());
}

#[test]
fn it_parses_respecting_parentesis_precedence() {
    let text = "(10+5)*4";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    let plusnode = test_node_builder(String::from("10"),
                                     String::from("+"),
                                     String::from("5"));

    let token = Token::build(Kind::Operator, String::from("*"));
    let rnode = ast::Node::leaf(Token::build(Kind::Integer, String::from("4")));

    let expected = ast::Node::new(Some(plusnode), token, Some(rnode));
    assert_eq!(expected, parser.parse());
}
