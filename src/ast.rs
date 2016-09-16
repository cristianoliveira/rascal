// This module contains the Abstract Sintax Tree representations

use token::{Token, Kind};

#[derive(Debug, Clone, PartialEq)]
pub enum Var {
    RString(String),
    RInteger(i32),
    RBoolean(bool),
    RFunction(Vec<Var>, Node),
    Nil
}
impl Var {
    pub fn from(token: Token) -> Var {
        match token {
            Token{kind: Kind::Integer, value} => {
                Var::RInteger(value.parse::<i32>().expect("Invalid integer value."))
            },
            Token{kind: Kind::Bolean, value} =>
                Var::RBoolean(value=="true"),
            _ => Var::Nil
        }
    }
    pub fn to_string(self) -> String {
        match self {
            Var::RFunction(_,_) => format!("function"),
            Var::RString(s) => format!("{}", s),
            Var::RInteger(s) => format!("{}", s),
            Var::RBoolean(s) => format!("{}", s),
            _ => String::new()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    locals: Vec<Var>,
    global: Vec<Var>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Identifier(String),
    Constant(Var),
    Binary(Node, String, Node),
    Comparison(Node, String, Node),
    DefineImut(Node, Node),
    DefineVar(Node, Node),
    ReAssign(Node, Node),
    NegUnary(Node),
    IfElse(Node, Node, Node),
    Loop(Node, Node),
    Block(Vec<Node>),
    Return(Node),
    Empty
}


// Node
//
// Represents a node inside of the tree
// each node must have an token and optional nodes
#[derive(Debug, Clone, PartialEq)]
pub struct Node{
    pub operation: Box<Operation>,
    pub token: Token,
}

impl Node {
    pub fn binary(left: Node, token: Token, right: Node) -> Self {
        Node {
            operation: Box::new(Operation::Binary(left, token.clone().value, right)),
            token: token,
        }
    }
    pub fn comparison(left: Node, token: Token, right: Node) -> Self {
        Node {
            operation: Box::new(Operation::Comparison(left, token.clone().value, right)),
            token: token,
        }
    }
    pub fn define_immutable(left: Node, right: Node) -> Self {
        Node {
            operation: Box::new(Operation::DefineImut(left.clone(), right.clone())),
            token: Token::build(Kind::Assign, String::from("=")),
        }
    }
    pub fn define_mutable(left: Node, right: Node) -> Self {
        Node {
            operation: Box::new(Operation::DefineVar(left.clone(), right.clone())),
            token: Token::build(Kind::Assign, String::from("=")),
        }
    }
    pub fn reassign(left: Node, right: Node) -> Self {
        Node {
            operation: Box::new(Operation::ReAssign(left.clone(), right.clone())),
            token: Token::build(Kind::ReAssign, String::new()),
        }
    }
    pub fn indentifier(token: Token) -> Self {
        Node {
            operation: Box::new(Operation::Identifier(token.clone().value)),
            token: token,
        }
    }
    pub fn constant(token: Token) -> Self {
        let primitive = Var::from(token.clone());
        Node {
            operation: Box::new(Operation::Constant(primitive)),
            token: token,
        }
    }
    pub fn unary(token: Token, node: Node) -> Self {
        Node {
            operation: Box::new(Operation::NegUnary(node.clone())),
            token: token,
        }
    }
    pub fn _return(node: Node) -> Self {
        Node {
            operation: Box::new(Operation::Return(node)),
            token: Token::build(Kind::Return, String::new()),
        }
    }
    pub fn ifelse(condition: Node, if_node: Node, else_node: Node) -> Self {
        Node {
            operation: Box::new(Operation::IfElse(condition, if_node, else_node)),
            token: Token::build(Kind::Conditional, String::new()),
        }
    }
    pub fn conditional(node:Node, statements: Node) -> Self {
        Node {
            operation: Box::new(Operation::Loop(node, statements)),
            token: Token::build(Kind::Conditional, String::new()),
        }
    }
    pub fn block(statements: Vec<Node>) -> Self {
        Node {
            operation: Box::new(Operation::Block(statements)),
            token: Token::build(Kind::Statement, String::new()),
        }
    }
    pub fn empty() -> Self {
        Node {
            operation: Box::new(Operation::Empty),
            token: Token::build(Kind::Empty, String::new()),
        }
    }
}
