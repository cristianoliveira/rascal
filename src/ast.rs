// This module contains the Abstract Sintax Tree representations

use token::{Token, Kind};

#[derive(Debug, Clone, PartialEq)]
pub enum Var {
    RString(String),
    RInteger(u32),
    RBoolean(bool),
    RFunction(Vec<Var>, Node)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    locals: Vec<Var>,
    global: Vec<Var>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Binary(Node, Node),
    DefineImut(Node, Node),
    DefineVar(Node, Node),
    ReAssign(Node, Node),
    NegUnary(Node),
    IfElse(Node, Node, Node),
    Loop(Node, Node),
    Block(Vec<Node>),
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
    pub statements: Option<Vec<Node>>,
    pub conditional: Box<Option<Node>>,
    pub left: Box<Option<Node>>,
    pub right: Box<Option<Node>>,
}

impl Node {
    pub fn binary(left: Node, token: Token, right: Node) -> Self {
        Node {
            operation: Box::new(Operation::Empty),
            left: Box::new(Some(left)),
            token: token,
            right: Box::new(Some(right)),
            conditional: Box::new(None),
            statements: None
        }
    }
    pub fn define_immutable(left: Node, right: Node) -> Self {
        Node {
            operation: Box::new(Operation::DefineImut(left.clone(), right.clone())),
            left: Box::new(Some(left)),
            token: Token::build(Kind::Assign, String::from("=")),
            right: Box::new(Some(right)),
            conditional: Box::new(None),
            statements: None
        }
    }
    pub fn define_mutable(left: Node, right: Node) -> Self {
        Node {
            operation: Box::new(Operation::DefineVar(left.clone(), right.clone())),
            left: Box::new(Some(left)),
            token: Token::build(Kind::Assign, String::from("=")),
            right: Box::new(Some(right)),
            conditional: Box::new(None),
            statements: None
        }
    }
    pub fn reassign(left: Node, right: Node) -> Self {
        Node {
            operation: Box::new(Operation::ReAssign(left.clone(), right.clone())),
            left: Box::new(Some(left)),
            token: Token::build(Kind::ReAssign, String::new()),
            right: Box::new(Some(right)),
            conditional: Box::new(None),
            statements: None
        }
    }
    pub fn leaf(token: Token) -> Self {
        Node {
            operation: Box::new(Operation::Empty),
            left: Box::new(None),
            token: token,
            right: Box::new(None),
            conditional: Box::new(None),
            statements: None
        }
    }
    pub fn unary(token: Token, node: Node) -> Self {
        Node {
            operation: Box::new(Operation::NegUnary(node.clone())),
            left: Box::new(None),
            token: token,
            right: Box::new(None),
            conditional: Box::new(None),
            statements: None
        }
    }
    pub fn _return(node: Node) -> Self {
        Node {
            operation: Box::new(Operation::Empty),
            left: Box::new(None),
            token: Token::build(Kind::Return, String::new()),
            right: Box::new(Some(node)),
            conditional: Box::new(None),
            statements: None
        }
    }
    pub fn ifelse(condition: Node, if_node: Node, else_node: Node) -> Self {
        Node {
            operation: Box::new(Operation::IfElse(condition, if_node, else_node)),
            left: Box::new(None),
            token: Token::build(Kind::Conditional, String::new()),
            right: Box::new(None),
            conditional: Box::new(None),
            statements: None
        }
    }
    pub fn conditional(node:Node, statements: Node) -> Self {
        Node {
            operation: Box::new(Operation::Loop(node, statements)),
            left: Box::new(None),
            token: Token::build(Kind::Conditional, String::new()),
            right: Box::new(None),
            conditional: Box::new(None),
            statements: None
        }
    }
    pub fn block(statements: Vec<Node>) -> Self {
        Node {
            operation: Box::new(Operation::Block(statements)),
            left: Box::new(None),
            token: Token::build(Kind::Statement, String::new()),
            right: Box::new(None),
            conditional: Box::new(None),
            statements: None
        }
    }
    pub fn empty() -> Self {
        Node {
            operation: Box::new(Operation::Empty),
            left: Box::new(None),
            token: Token::build(Kind::Empty, String::new()),
            right: Box::new(None),
            conditional: Box::new(None),
            statements: None
        }
    }
}
