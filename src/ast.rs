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
    left: Box<Option<Node>>,
    right: Box<Option<Node>>,
}

impl Node {
    pub fn new(left: Node, token: Token, right: Node) -> Self {
        Node {
            left: Box::new(Some(left)),
            token: token,
            right: Box::new(Some(right)),
            statements: None
        }
    }
    pub fn leaf(token: Token) -> Self {
        Node {
            left: Box::new(None),
            token: token,
            right: Box::new(None),
            statements: None
        }
    }
    pub fn unary(token: Token, node: Node) -> Self {
        Node {
            left: Box::new(None),
            token: token,
            right: Box::new(Some(node)),
            statements: None
        }
    }
    pub fn _return(node: Node) -> Self {
        Node {
            left: Box::new(None),
            token: Token::build(Kind::Return, String::new()),
            right: Box::new(Some(node)),
            statements: None
        }
    }
    pub fn compound(statements: Vec<Node>) -> Self {
        Node {
            left: Box::new(None),
            token: Token::build(Kind::Statement, String::new()),
            right: Box::new(None),
            statements: Some(statements)
        }
    }
    pub fn empty() -> Self {
        Node {
            left: Box::new(None),
            token: Token::build(Kind::Statement, String::new()),
            right: Box::new(None),
            statements: None
        }
    }
    pub fn nodes(self) -> (Option<Node>, Option<Node>) {
        (*self.left, *self.right)
    }
    pub fn value(self) -> String {
        self.token.value
    }
    pub fn kind(self) -> Kind {
        self.token.kind
    }
}
