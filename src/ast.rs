// This module contains the Abstract Sintax Tree representations

use token::{Token, Kind};
use std::collections::HashMap;

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
    pub iparents: HashMap<String, String>,
    pub parents: HashMap<String, String>,
    pub ilocals: HashMap<String, String>,
    pub locals: HashMap<String, String>,
}
impl Frame {
    pub fn new() -> Self {
        Frame {
            locals: HashMap::new(),
            parents: HashMap::new(),
            ilocals: HashMap::new(),
            iparents: HashMap::new()
        }
    }

    pub fn has(&self, id: &str) -> bool {
        self.ilocals.contains_key(id) || self.locals.contains_key(id)
    }

    pub fn is_imutable(&self, id: &str) -> bool {
        self.ilocals.contains_key(id)
    }

    pub fn get(&self, id: &str) -> String {
        // parent
        if let Some(value) = self.iparents.get(&*id) { return value.clone() };
        if let Some(value) = self.parents.get(&*id) { return value.clone() };
        // current
        if let Some(value) = self.ilocals.get(&*id) { return value.clone() };
        if let Some(value) = self.locals.get(&*id) {
            return value.clone()
        } else {
            panic!("Variable {} doesn't exists in this context", id)
        }
    }
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
    pub frame: Frame,
    pub operation: Box<Operation>,
    pub value: String,
}

impl Node {
    pub fn binary(left: Node, token: Token, right: Node) -> Self {
        Node {
            frame: Frame::new(),
            operation: Box::new(Operation::Binary(left, token.clone().value, right)),
            value: token.value,
        }
    }
    pub fn comparison(left: Node, token: Token, right: Node) -> Self {
        Node {
            frame: Frame::new(),
            operation: Box::new(Operation::Comparison(left, token.clone().value, right)),
            value: token.value,
        }
    }
    pub fn define_immutable(left: Node, right: Node) -> Self {
        Node {
            frame: Frame::new(),
            operation: Box::new(Operation::DefineImut(left.clone(), right.clone())),
            value: String::from("=")
        }
    }
    pub fn define_mutable(left: Node, right: Node) -> Self {
        Node {
            frame: Frame::new(),
            operation: Box::new(Operation::DefineVar(left.clone(), right.clone())),
            value: String::from("=")
        }
    }
    pub fn reassign(left: Node, right: Node) -> Self {
        Node {
            frame: Frame::new(),
            operation: Box::new(Operation::ReAssign(left.clone(), right.clone())),
            value: String::from("")
        }
    }
    pub fn indentifier(token: Token) -> Self {
        Node {
            frame: Frame::new(),
            operation: Box::new(Operation::Identifier(token.clone().value)),
            value: token.value
        }
    }
    pub fn constant(token: Token) -> Self {
        let primitive = Var::from(token.clone());
        Node {
            frame: Frame::new(),
            operation: Box::new(Operation::Constant(primitive)),
            value: token.value
        }
    }
    pub fn unary(token: Token, node: Node) -> Self {
        Node {
            frame: Frame::new(),
            operation: Box::new(Operation::NegUnary(node.clone())),
            value: token.value
        }
    }
    pub fn _return(node: Node) -> Self {
        Node {
            frame: Frame::new(),
            operation: Box::new(Operation::Return(node)),
            value: String::new()
        }
    }
    pub fn ifelse(condition: Node, if_node: Node, else_node: Node) -> Self {
        Node {
            frame: Frame::new(),
            operation: Box::new(Operation::IfElse(condition, if_node, else_node)),
            value: String::new()
        }
    }
    pub fn conditional(node:Node, statements: Node) -> Self {
        Node {
            frame: Frame::new(),
            operation: Box::new(Operation::Loop(node, statements)),
            value: String::new()
        }
    }
    pub fn block(statements: Vec<Node>) -> Self {
        Node {
            frame: Frame::new(),
            operation: Box::new(Operation::Block(statements)),
            value: String::new()
        }
    }
    pub fn empty() -> Self {
        Node {
            frame: Frame::new(),
            operation: Box::new(Operation::Empty),
            value: String::new()
        }
    }
}
