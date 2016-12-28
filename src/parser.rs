use token::{Token, Kind, Tokenizer};
use ast;

// # Parser
//
// Represents the parser that is responsible for parsing a stream of tokens
// from a given Tokenizer into an Abstracted Sintax Tree
//
// Context free grammar for this language:
//```
//
//   block: BEGIN statement_list END
//
//   statement_list: statement
//   statement_list: statement STATEMENT_END statement_list
//
//   statement: block
//   statement: return_statement
//   statement: while
//   statement: if
//   statement: define_statement
//   statement: assign_statement
//   statement: empty_statement
//
//   assign_statement: constant ASSIGN expr
//
//   while: WHILE expr BEGIN statement_list END
//
//   if: IF expr BEGIN statement_list END
//   if: IF expr BEGIN statement_list ELSE statement_lit END
//
//   define_statement: FUN variable ASSIGN ( params_list ) block
//   define_statement: MUT variable
//   define_statement: MUT variable ASSIGN expr
//   define_statement: IMUT constant ASSIGN expr
//
//   expr:: term
//   expr:: term (+|-) term
//   expr:: term (OR) term
//   expr:: term (==|!==|>|<) term
//
//   term:: factor
//   term:: factor (*|/) factor
//   term:: factor (AND) factor
//
//   factor:: (-|+) factor
//   factor:: ( expr )
//   factor:: INTEGER
//   factor:: BOOLEAN
//   factor:: variable
//   factor:: function_call
//
//   variable:: ID
//```
pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(lexer: Tokenizer) -> Self {
        Parser { tokenizer: lexer }
    }

    // function_call
    //
    // function_call is a ID followed by ( followed by )
    // Represented as context free grammar:
    // ```
    //   function_call: ID BEGIN END
    //   function_call: ID BEGIN param_list END
    // ```
    fn function_call(&mut self) -> ast::Node {
        let name = self.variable();
        self.tokenizer.advance().consume(Kind::GroupBegin);
        let args = self.args_list();
        self.tokenizer.advance().consume(Kind::GroupEnd);
        ast::Node::call_function(name, args)
    }


    // block
    //
    // block is a BEGIN followed by statement_list followed by END
    // Represented as context free grammar:
    // ```
    //   block: BEGIN statement_list END
    // ```
    fn block(&mut self) -> ast::Node {
        self.tokenizer.advance().consume(Kind::Begin);
        let statement_list = self.statement_list();
        self.tokenizer.advance().consume(Kind::End);
        ast::Node::block(statement_list)
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
    //   statement: block
    //   statement: return_statement
    //   statement: while_block
    //   statement: if_block
    //   statement: define_statement
    //   statement: assign_statement
    //   statement: std_output_statement
    //   statement: empty_statement
    // ```
    fn statement(&mut self) -> ast::Node {
        let curr = self.tokenizer.advance().get();
        match curr {
            Some(Token{ kind: Kind::Return, ..}) => {
                self.tokenizer.consume(Kind::Return);
                ast::Node::_return(self.expr())
            },
            Some(Token{ kind: Kind::StdOut, ..}) => {
                self.tokenizer.consume(Kind::StdOut);
                ast::Node::print(self.expr())
            },
            Some(Token{ kind: Kind::ImmutableDefine, ..}) |
            Some(Token{ kind: Kind::FunctionDefine, ..}) |
            Some(Token{ kind: Kind::MutableDefine, ..}) => {
                self.define_statement()
            },
            Some(Token{ kind: Kind::ID, ..}) => {
                let next = self.tokenizer.peek(0);
                match next {
                    Some(Token{kind: Kind::GroupBegin, ..}) =>
                        self.function_call(),

                    Some(Token{kind: Kind::Assign, ..}) =>
                        self.assign_statement(),

                    _ => self.expr()
                }
            },
            Some(Token{ kind: Kind::Begin, ..}) => self.block(),
            Some(Token{ kind: Kind::While, ..}) => self._while(),
            Some(Token{ kind: Kind::If, ..}) => self._if(),
            _ => self.expr()
        }
    }

    // assign_statement
    //
    // assign_statement is an constant followed by an assign token followed by
    // an expression (expr). Represented as context free grammar:
    // ```
    //   assign_statement: constant ASSIGN expr
    // ```
    fn assign_statement(&mut self) -> ast::Node {
        let (name, _, expr) = (self.variable(),
            self.tokenizer.advance().consume(Kind::Assign).value,
            self.expr()
        );
        ast::Node::reassign(name, expr)
    }

    // while
    //
    // while is a BEGIN followed by statement_list followed by END
    // Represented as context free grammar:
    // ```
    //   while: WHILE expr BEGIN statement_list END
    // ```
    fn _while(&mut self) -> ast::Node {
        self.tokenizer.consume(Kind::While);
        let conditional = self.expr();
        let block = self.block();
        ast::Node::conditional(conditional, block)
    }

    // if
    //
    // if is a IF BEGIN followed by statement_list followed optionally by
    // ELSE followed by statement_list followed by END
    // Represented as context free grammar:
    // ```
    //   if: IF expr BEGIN statement_list END
    //   if: IF expr BEGIN statement_list ELSE statement_lit END
    // ```
    fn _if(&mut self) -> ast::Node {
        self.tokenizer.consume(Kind::If);
        let condition = self.expr();
        self.tokenizer.consume(Kind::Begin);
        let if_node = ast::Node::block(self.statement_list());
        let optional_elsenode =
            if let Some(Token{ kind: Kind::Else, ..}) = self.tokenizer.get() {
                self.tokenizer.consume(Kind::Else);
                ast::Node::block(self.statement_list())
            } else {
                ast::Node::empty()
            };
        self.tokenizer.consume(Kind::End);
        ast::Node::ifelse(condition, if_node, optional_elsenode)
    }

    // define_statement
    //
    // define_statement is an LET or MUT followed by variable followed by
    // an assign token followed by an expression (expr).
    // Represented as context free grammar:
    // ```
    //   define_statement: FUN variable ASSIGN ( params_list ) block
    //   define_statement: MUT variable
    //   define_statement: MUT variable ASSIGN expr
    //   define_statement: IMUT constant ASSIGN expr
    // ```
    fn define_statement(&mut self) -> ast::Node {
        match self.tokenizer.get() {
            Some(Token{ kind: Kind::ImmutableDefine, ..}) => {
                self.tokenizer.consume(Kind::ImmutableDefine);
                self.tokenizer.advance();
                let (var, _, expr) = (
                    self.constant(),
                    self.tokenizer.advance().consume(Kind::Assign),
                    self.expr()
                 );
                ast::Node::define_immutable(var, expr)
            },
            Some(Token{ kind: Kind::MutableDefine, ..}) => {
                self.tokenizer.consume(Kind::MutableDefine);
                self.tokenizer.advance();
                let (var, _, expr) = (
                    self.constant(),
                    self.tokenizer.advance().consume(Kind::Assign),
                    self.expr()
                 );
                ast::Node::define_mutable(var, expr)
            },
            Some(Token{ kind: Kind::FunctionDefine, ..}) => {
                self.tokenizer.consume(Kind::FunctionDefine);
                self.tokenizer.advance();

                let var = self.variable();
                self.tokenizer.advance().consume(Kind::Assign);

                self.tokenizer.advance().consume(Kind::FunctionParamBegin);
                let params = self.params_list();
                self.tokenizer.advance().consume(Kind::FunctionParamEnd);

                let block = self.block();

                ast::Node::define_function(var, params, block)
            },
            _ => ast::Node::empty()
        }
    }

    // constant
    //
    // constant is an CONST. Represented as context free grammar:
    // ```
    //   constant: CONST
    // ```
    fn constant(&mut self) -> ast::Node {
        let token = self.tokenizer.consume(Kind::ID);
        ast::Node::indentifier(Token::build(Kind::CONST, token.value))
    }

    // variable
    //
    // variable is an ID. Represented as context free grammar:
    // ```
    //   variable: ID
    // ```
    fn variable(&mut self) -> ast::Node {
        let token = self.tokenizer.advance().consume(Kind::ID);
        ast::Node::indentifier(token)
    }

    // args_list
    // args_list is an ID that can be followed by SEPARATOR followed by ID
    // Represented as context free grammar:
    // ```
    //   args_list: expr
    //   args_list: expr SEPARATOR expr]
    // ```
    fn args_list(&mut self) -> Vec<ast::Node> {
        let mut args = vec![];
        match self.tokenizer.advance().get() {
            Some(Token{kind: Kind::GroupEnd, ..}) => {
                return args
            },
            Some(Token{kind: Kind::Separator, ..}) => {
                self.tokenizer.consume(Kind::Separator);
            },
            _ => {
                args.push(self.factor())
            }
        }
        args.extend(self.args_list());
        args
    }

    // params_list
    // params_list is an ID that can be followed by SEPARATOR followed by ID
    // Represented as context free grammar:
    // ```
    //   params_list: [ID]
    //   params_list: [ID SEPARATOR params_list]
    // ```
    fn params_list(&mut self) -> Vec<ast::Node> {
        let mut params = vec![];
        if let Some(Token{kind: Kind::ID, ..}) = self.tokenizer.advance().get() {
            params.push(self.variable());
            while let Some(Token{kind: Kind::Separator, ..}) = self.tokenizer.advance().get() {
                self.tokenizer.consume(Kind::Separator);
                params.push(self.variable())
            }
        }
        params
    }

    // factor
    //
    // factor can be a terminal Integer, result of a grouped expr,
    // unary result of a factor or a var. Represented as context free grammar:
    // ```
    //  factor:: (-|+) factor
    //  factor:: INTEGER
    //  factor:: BOLEAN
    //  factor:: ( expr )
    //  factor:: variable
    //  factor:: constant
    //  factor:: function_call
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
                ast::Node::constant(self.tokenizer.advance().consume(Kind::Integer))
            },

            Some(Token{ kind: Kind::Bolean, .. }) => {
                ast::Node::constant(self.tokenizer.advance().consume(Kind::Bolean))
            },

            Some(Token{ kind: Kind::ID, .. }) => {
                if let Some(Token{kind: Kind::GroupBegin, ..}) = self.tokenizer.peek(1) {
                    self.function_call()
                } else {
                    self.variable()
                }
            },

            Some(Token{ kind: Kind::CONST, .. }) => {
                self.constant()
            },

            None => ast::Node::empty(),

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
    // term:: factor (!== | == | > | <) factor
    // ```
    fn term(&mut self) -> ast::Node {
        let result = self.factor();

        if let Some(token) = self.tokenizer.advance().get() {
            match token.value.as_ref() {
                "*" | "/" | "%" => {
                    return ast::Node::binary(
                        result.clone(),
                        self.tokenizer.consume(Kind::Operator).value,
                        self.factor())
                },
                "==" | "!=" | ">" | "<" => {
                    return ast::Node::comparison(
                        result.clone(),
                        self.tokenizer.consume(Kind::Comparison).value,
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
                    result = ast::Node::binary(result.clone(),
                                            self.tokenizer.consume(Kind::Operator).value,
                                            self.term())
                },
                "and"|"&&"|"or" | "||" => {
                    result = ast::Node::comparison(
                        result.clone(),
                        self.tokenizer.consume(Kind::Comparison).value,
                        self.term())
                },
                _ => break
            };
        }
        result
    }

    pub fn parse(&mut self) -> ast::Node {
        ast::Node::main(self.statement_list())
    }
}


#[allow(dead_code)]
fn test_node_builder(left: String, operator: String, right: String) -> ast::Node {
    let lnode = ast::Node::constant(Token::build(Kind::Integer, left));
    let token = Token::build(Kind::Operator, operator);
    let rnode = ast::Node::constant(Token::build(Kind::Integer, right));
    ast::Node::binary(lnode, token.value, rnode)
}

#[test]
fn it_parses_sum_as_node() {
    let text = "5+1";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);
    let expected = test_node_builder(String::from("5"),
                                     String::from("+"),
                                     String::from("1"));

    assert_eq!(ast::Node::main(vec![expected]), parser.parse());
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
    let rnode = ast::Node::constant(Token::build(Kind::Integer, String::from("4")));

    let expected = ast::Node::binary(firstsum, token.value, rnode);
    assert_eq!(ast::Node::main(vec![expected]), parser.parse());
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
    let rnode = ast::Node::constant(Token::build(Kind::Integer, String::from("10")));

    let expected = ast::Node::binary(rnode, token.value, plusnode);
    assert_eq!(ast::Node::main(vec![expected]), parser.parse());
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
    let rnode = ast::Node::constant(Token::build(Kind::Integer, String::from("4")));

    let expected = ast::Node::binary(plusnode, token.value, rnode);
    assert_eq!(ast::Node::main(vec![expected]), parser.parse());
}

#[test]
fn it_parses_simple_block() {
    let text = "{ var x = 10+5 }";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    let expr = test_node_builder(String::from("10"),
                                 String::from("+"),
                                 String::from("5"));

    let var = ast::Node::indentifier(Token{ kind: Kind::ID, value: String::from("x")});
    let assign = ast::Node::define_mutable(var, expr);

    let expected = ast::Node::block(vec![assign]);
    assert_eq!(ast::Node::main(vec![expected]), parser.parse());
}

#[test]
fn it_parses_block_single_expression() {
    let text = "{ 10 + 5 }";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    let expr = test_node_builder(String::from("10"),
                                 String::from("+"),
                                 String::from("5"));

    let expected = ast::Node::block(vec![expr]);
    assert_eq!(ast::Node::main(vec![expected]), parser.parse());
}

#[test]
fn it_parses_multiple_statements() {
    let text = "var x = 10+5; let y = 100";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    let yvar = ast::Node::indentifier(Token{ kind: Kind::CONST, value: String::from("y")});
    let yvalue = ast::Node::constant(Token{ kind: Kind::Integer, value: String::from("100")});
    let yassign = ast::Node::define_immutable(yvar, yvalue);

    let expr = test_node_builder(String::from("10"),
                                 String::from("+"),
                                 String::from("5"));
    let xvar = ast::Node::indentifier(Token{ kind: Kind::ID, value: String::from("x")});
    let xassign = ast::Node::define_mutable(xvar, expr);

    let expected = ast::Node::main(vec![xassign, yassign]);
    assert_eq!(expected, parser.parse());
}

#[test]
fn it_parses_bolean_comparison() {
    let text = "true == false";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    let lcompar = ast::Node::constant(Token::build(Kind::Bolean, String::from("true")));
    let rcompar = ast::Node::constant(Token::build(Kind::Bolean, String::from("false")));
    let tkcompar = Token::build(Kind::Comparison, String::from("=="));
    let statement = ast::Node::comparison(lcompar, tkcompar.value, rcompar);

    let expected = ast::Node::main(vec![statement]);
    assert_eq!(expected, parser.parse());
}

#[test]
fn it_parses_bolean_expression() {
    let text = "true and true == false";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    let lcompar = ast::Node::constant(Token::build(Kind::Bolean, String::from("true")));
    let rcompar = ast::Node::constant(Token::build(Kind::Bolean, String::from("false")));
    let tkcompar = Token::build(Kind::Comparison, String::from("=="));
    let comparison = ast::Node::comparison(lcompar, tkcompar.value, rcompar);

    let token = Token::build(Kind::Comparison, String::from("and"));
    let rnode = ast::Node::constant(Token::build(Kind::Bolean, String::from("true")));

    let statement = ast::Node::comparison(rnode, token.value, comparison);
    let expected = ast::Node::main(vec![statement]);
    assert_eq!(expected, parser.parse());
}

#[test]
fn it_parses_expressions_gt_lt() {
    let text = "1 > 2 or 1 < 2";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    let lcompar = ast::Node::constant(Token::build(Kind::Integer, String::from("1")));
    let tkcompar = Token::build(Kind::Comparison, String::from(">"));
    let rcompar = ast::Node::constant(Token::build(Kind::Integer, String::from("2")));
    let lnode = ast::Node::comparison(lcompar, tkcompar.value, rcompar);

    let token = Token::build(Kind::Comparison, String::from("or"));

    let lcompar2 = ast::Node::constant(Token::build(Kind::Integer, String::from("1")));
    let tkcompa2 = Token::build(Kind::Comparison, String::from("<"));
    let rcompar2 = ast::Node::constant(Token::build(Kind::Integer, String::from("2")));
    let rnode = ast::Node::comparison(lcompar2, tkcompa2.value, rcompar2);

    let expected = ast::Node::comparison(lnode, token.value, rnode);
    assert_eq!(ast::Node::main(vec![expected]), parser.parse());
}


#[test]
fn it_parses_function_define() {
    let text = "{ fn x = [arg, arg2] { return arg + arg2 } }";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    let fun = ast::Node::indentifier(Token{ kind: Kind::ID, value: String::from("x")});
    let arg = ast::Node::indentifier(Token{ kind: Kind::ID, value: String::from("arg")});
    let arg2 = ast::Node::indentifier(Token{ kind: Kind::ID, value: String::from("arg2")});
    let params = vec![arg, arg2];

    let arg1 = ast::Node::indentifier(Token{ kind: Kind::ID, value: String::from("arg")});
    let plus = Token::build(Kind::Operator, String::from("+"));
    let arg2 = ast::Node::indentifier(Token{ kind: Kind::ID, value: String::from("arg2")});
    let expr = ast::Node::binary(arg1, plus.value, arg2);
    let nreturn = ast::Node::_return(expr);

    let block = ast::Node::block(vec![nreturn]);

    let fundefine = ast::Node::define_function(fun, params, block);
    let program = ast::Node::block(vec![fundefine]);
    assert_eq!(ast::Node::main(vec![program]), parser.parse());
}

