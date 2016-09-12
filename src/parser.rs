use token::{Token, Kind, Tokenizer};
use ast;

// # Parser
//
// Represents the parser that is responsible for parsing a stream of tokens
// from a given Tokenizer into an Abstracted Sintax Tree
//
// Context free grammar for this language:
//```
//   expr:: term
//   expr:: term (+|-) term
//   expr:: term (OR) term
//
//   term:: factor
//   term:: factor (*|/) factor
//   term:: factor (AND) factor
//
//   factor:: (-|+) factor
//   factor:: (==|!=) factor
//   factor:: ( expr )
//   factor:: INTEGER
//   factor:: BOOLEAN
//   factor:: variable
//
//   variable:: ID
//```
pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(lexer: Tokenizer) -> Self {
        Parser {
            tokenizer: lexer,
        }
    }

    // while
    //
    // while is a BEGIN followed by statement_list followed by END
    // Represented as context free grammar:
    // ```
    //   while: expr BEGIN statement_list END
    // ```
    fn _while(&mut self) -> ast::Node {
        self.tokenizer.consume(Kind::While);
        let conditional = self.expr();
        self.tokenizer.consume(Kind::Begin);
        let statement_list = self.statement_list();
        self.tokenizer.consume(Kind::End);
        ast::Node::conditional(conditional, statement_list)
    }

    // compound
    //
    // compound is a BEGIN followed by statement_list followed by END
    // Represented as context free grammar:
    // ```
    //   compound: BEGIN statement_list END
    // ```
    fn compound(&mut self) -> ast::Node {
        self.tokenizer.consume(Kind::Begin);
        let statement_list = self.statement_list();
        self.tokenizer.consume(Kind::End);
        ast::Node::compound(statement_list)
    }

    // statement_list
    //
    // statement_list can be a single statement or single statement followed by
    // STATEMENT_END followed by statement_list recursively
    // Represented as context free grammar:
    // ```
    //   statement_list: statement
    //   statement_list: statement STATEMENT_END statement_list
    // ```
    fn statement_list(&mut self) -> Vec<ast::Node> {
        let mut statements = vec![self.statement()];
        if let Some(Token{kind: Kind::StatementEnd, ..}) = self.tokenizer.advance().get() {
            self.tokenizer.consume(Kind::StatementEnd);
            statements.extend(self.statement_list())
        }
        return statements
    }

    // statement
    //
    // statement can be a compoud, assign or empty statement.
    // Represented as context free grammar:
    // ```
    //   statement: compoud_statement
    //   statement: return_statement
    //   statement: assign_statement
    //   statement: empty_statement
    // ```
    fn statement(&mut self) -> ast::Node {
        match self.tokenizer.advance().get() {
            Some(Token{ kind: Kind::Return, ..}) => {
                self.tokenizer.consume(Kind::Return);
                ast::Node::_return(self.expr())
            },
            Some(Token{ kind: Kind::ID, ..}) => {
                self.assign_statement()
            },
            Some(Token{ kind: Kind::Begin, ..}) => self.compound(),
            Some(Token{ kind: Kind::While, ..}) => self._while(),
            _ => ast::Node::empty()
        }
    }

    // assign_statement
    //
    // assign_statement is an variable followed by an assign token followed by 
    // an expression (expr). Represented as context free grammar:
    // ```
    //   assign_statement: variable ASSIGN expr
    // ```
    fn assign_statement(&mut self) -> ast::Node {
        ast::Node::new(self.variable(),
                       self.tokenizer.advance().consume(Kind::Assign),
                       self.expr())
    }

    // variable
    //
    // variable is an ID. Represented as context free grammar:
    // ```
    //   variable: ID
    // ```
    fn variable(&mut self) -> ast::Node {
        let token = self.tokenizer.consume(Kind::ID);
        ast::Node::leaf(token)
    }

    // factor
    //
    // factor can be a terminal Integer, result of a grouped expr,
    // unary result of a factor or a var. Represented as context free grammar:
    // ```
    //  factor:: (-|+) factor
    //  factor:: (==|!=|>|<) factor
    //  factor:: INTEGER
    //  factor:: BOLEAN
    //  factor:: ( expr )
    //  factor:: variable
    // ```
    fn factor(&mut self) -> ast::Node {
        match self.tokenizer.advance().get() {
            Some(Token{ kind: Kind::Operator, .. }) => {
                ast::Node::unary(self.tokenizer.consume(Kind::Operator), self.factor())
            },

            Some(Token{ kind: Kind::GroupBegin , .. }) => {
                self.tokenizer.consume(Kind::GroupBegin);
                let result = self.expr();
                self.tokenizer.consume(Kind::GroupEnd);
                result
            },

            Some(Token{ kind: Kind::Integer, .. }) => {
                ast::Node::leaf(self.tokenizer.advance().consume(Kind::Integer))
            },

            Some(Token{ kind: Kind::Bolean, .. }) => {
                ast::Node::leaf(self.tokenizer.advance().consume(Kind::Bolean))
            },

            Some(Token{ kind: Kind::ID, .. }) => {
                self.variable()
            },

            other =>
            panic!("Factor error: exptected Operator|GroupBegin|Integer|ID
                   found {:?}", other)
        }
    }

    // term
    //
    // One term can be a `factor` or result of `factor * factor`
    // or `factor / factor`. Represented in context free grammar:
    // ```
    // term:: factor
    // term:: factor (*|/) factor
    // term:: factor (!== | ==) factor
    // ```
    fn term(&mut self) -> ast::Node {
        let result = self.factor();

        if let Some(token) = self.tokenizer.advance().get() {
            match token.value.as_ref() {
                "*" | "/" => {
                    return ast::Node::new(result.clone(),
                                          self.tokenizer.consume(Kind::Operator),
                                          self.factor())
                },
                "==" | "!=" | ">" | "<" => {
                    return ast::Node::new(result.clone(),
                                          self.tokenizer.consume(Kind::Comparison),
                                          self.factor())
                },
                _ => ()
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
    //   expr:: term (OR|AND) term
    // ```
    pub fn expr(&mut self) -> ast::Node {
        let mut result = self.term();
        while let Some(token) = self.tokenizer.advance().get() {
            if token.kind == Kind::EOF { break }
            match token.value.as_ref() {
                "+" | "-" => {
                    result = ast::Node::new(result.clone(),
                                            self.tokenizer.consume(Kind::Operator),
                                            self.term())
                },
                "and"|"&&"|"or" | "||" => {
                    result = ast::Node::new(result.clone(),
                                            self.tokenizer.consume(Kind::Comparison),
                                            self.term())
                },
                _ => break
            };
        }
        result
    }

    pub fn parse(&mut self) -> ast::Node {
        match self.tokenizer.advance().get() {
            Some(Token{kind: Kind::Begin, ..}) => self.compound(),
            _ => self.expr()
        }
    }
}


fn test_node_builder(left: String, operator: String, right: String) -> ast::Node {
    let lnode = ast::Node::leaf(Token::build(Kind::Integer, left));
    let token = Token::build(Kind::Operator, operator);
    let rnode = ast::Node::leaf(Token::build(Kind::Integer, right));
    ast::Node::new(lnode, token, rnode)
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

    let expected = ast::Node::new(firstsum, token, rnode);
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

    let expected = ast::Node::new(rnode, token, plusnode);
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

    let expected = ast::Node::new(plusnode, token, rnode);
    assert_eq!(expected, parser.parse());
}

#[test]
fn it_parses_simple_block() {
    let text = "begin x = 10+5 end";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    let expr = test_node_builder(String::from("10"),
                                 String::from("+"),
                                 String::from("5"));

    let var = ast::Node::leaf(Token{ kind: Kind::ID, value: String::from("x")});
    let assign_token = Token{ kind: Kind::Assign, value: String::from("=")};
    let assign = ast::Node::new(var, assign_token, expr);

    let comp = ast::Node::compound(vec![assign]);
    assert_eq!(comp, parser.parse());
}

#[test]
fn it_parses_multiple_statements() {
    let text = "begin x = 10+5; y = 100 end";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);
    let assign_token = Token{ kind: Kind::Assign, value: String::from("=")};


    let yvar = ast::Node::leaf(Token{ kind: Kind::ID, value: String::from("y")});
    let yvalue = ast::Node::leaf(Token{ kind: Kind::Integer, value: String::from("100")});
    let yassign = ast::Node::new(yvar, assign_token.clone(), yvalue);

    let expr = test_node_builder(String::from("10"),
                                 String::from("+"),
                                 String::from("5"));
    let xvar = ast::Node::leaf(Token{ kind: Kind::ID, value: String::from("x")});
    let xassign = ast::Node::new(xvar, assign_token, expr);

    let comp = ast::Node::compound(vec![xassign, yassign]);
    assert_eq!(comp, parser.parse());
}

#[test]
fn it_parses_bolean_comparison() {
    let text = "true == false";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    let lcompar = ast::Node::leaf(Token::build(Kind::Bolean, String::from("true")));
    let rcompar = ast::Node::leaf(Token::build(Kind::Bolean, String::from("false")));
    let tkcompar = Token::build(Kind::Comparison, String::from("=="));
    let comparison = ast::Node::new(lcompar, tkcompar, rcompar);

    assert_eq!(comparison, parser.parse());
}

#[test]
fn it_parses_bolean_expression() {
    let text = "true and true == false";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    let lcompar = ast::Node::leaf(Token::build(Kind::Bolean, String::from("true")));
    let rcompar = ast::Node::leaf(Token::build(Kind::Bolean, String::from("false")));
    let tkcompar = Token::build(Kind::Comparison, String::from("=="));
    let comparison = ast::Node::new(lcompar, tkcompar, rcompar);

    let token = Token::build(Kind::Comparison, String::from("and"));
    let rnode = ast::Node::leaf(Token::build(Kind::Bolean, String::from("true")));

    let expected = ast::Node::new(rnode, token, comparison);
    assert_eq!(expected, parser.parse());
}

#[test]
fn it_parses_expressions_gt_lt() {
    let text = "1 > 2 or 1 < 2";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    let lcompar = ast::Node::leaf(Token::build(Kind::Integer, String::from("1")));
    let tkcompar = Token::build(Kind::Comparison, String::from(">"));
    let rcompar = ast::Node::leaf(Token::build(Kind::Integer, String::from("2")));
    let lnode = ast::Node::new(lcompar, tkcompar, rcompar);

    let token = Token::build(Kind::Comparison, String::from("or"));

    let lcompar2 = ast::Node::leaf(Token::build(Kind::Integer, String::from("1")));
    let tkcompa2 = Token::build(Kind::Comparison, String::from("<"));
    let rcompar2 = ast::Node::leaf(Token::build(Kind::Integer, String::from("2")));
    let rnode = ast::Node::new(lcompar2, tkcompa2, rcompar2);

    let expected = ast::Node::new(lnode, token, rnode);
    assert_eq!(expected, parser.parse());
}


