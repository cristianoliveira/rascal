// This module contains the Abstract Sintax Tree representations

use token::{Token, Kind};

// Node
//
// Represents a node inside of the tree
// each node must have an token and optional nodes
#[derive(Debug, Clone, PartialEq)]
pub struct Node{
    pub token: Token,
    pub statements: Option<Vec<Node>>,
    pub conditional: Box<Option<Node>>,
    pub left: Box<Option<Node>>,
    pub right: Box<Option<Node>>,
}

impl Node {
    pub fn binary(left: Node, token: Token, right: Node) -> Self {
        Node {
            left: Box::new(Some(left)),
            token: token,
            right: Box::new(Some(right)),
            conditional: Box::new(None),
            statements: None
        }
    }
    pub fn define_immutable(left: Node, right: Node) -> Self {
        Node {
            left: Box::new(Some(left)),
            token: Token::build(Kind::Assign, String::from("=")),
            right: Box::new(Some(right)),
            conditional: Box::new(None),
            statements: None
        }
    }
    pub fn define_mutable(left: Node, right: Node) -> Self {
        Node {
            left: Box::new(Some(left)),
            token: Token::build(Kind::Assign, String::from("=")),
            right: Box::new(Some(right)),
            conditional: Box::new(None),
            statements: None
        }
    }
    pub fn reassign(left: Node, right: Node) -> Self {
        Node {
            left: Box::new(Some(left)),
            token: Token::build(Kind::ReAssign, String::new()),
            right: Box::new(Some(right)),
            conditional: Box::new(None),
            statements: None
        }
    }
    pub fn leaf(token: Token) -> Self {
        Node {
            left: Box::new(None),
            token: token,
            right: Box::new(None),
            conditional: Box::new(None),
            statements: None
        }
    }
    pub fn unary(token: Token, node: Node) -> Self {
        Node {
            left: Box::new(None),
            token: token,
            right: Box::new(Some(node)),
            conditional: Box::new(None),
            statements: None
        }
    }
    pub fn _return(node: Node) -> Self {
        Node {
            left: Box::new(None),
            token: Token::build(Kind::Return, String::new()),
            right: Box::new(Some(node)),
            conditional: Box::new(None),
            statements: None
        }
    }
    pub fn ifelse(condition: Node, if_node: Node, else_node: Node) -> Self {
        Node {
            left: Box::new(Some(if_node)),
            token: Token::build(Kind::Conditional, String::new()),
            right: Box::new(Some(else_node)),
            conditional: Box::new(Some(condition)),
            statements: None
        }
    }
    pub fn conditional(node:Node, statements: Vec<Node>) -> Self {
        Node {
            left: Box::new(None),
            token: Token::build(Kind::Conditional, String::new()),
            right: Box::new(None),
            conditional: Box::new(Some(node)),
            statements: Some(statements)
        }
    }
    pub fn block(statements: Vec<Node>) -> Self {
        Node {
            left: Box::new(None),
            token: Token::build(Kind::Statement, String::new()),
            right: Box::new(None),
            conditional: Box::new(None),
            statements: Some(statements)
        }
    }
    pub fn empty() -> Self {
        Node {
            left: Box::new(None),
            token: Token::build(Kind::Empty, String::new()),
            right: Box::new(None),
            conditional: Box::new(None),
            statements: None
        }
    }
}
