// This module contains the Abstract Sintax Tree representations

use token::{Token, Kind};

// Node
//
// Represents a node inside of the tree
// each node must have an token and optional nodes
#[derive(Debug, Clone, PartialEq)]
pub struct Node{
    pub token: Token,
    left: Box<Option<Node>>,
    right: Box<Option<Node>>,
}

impl Node {
    pub fn new(left: Option<Node>, token: Token, right: Option<Node>) -> Self {
        Node {
            left: Box::new(left),
            token: token,
            right: Box::new(right),
        }
    }
    pub fn leaf(token: Token) -> Self {
        Node {
            left: Box::new(None),
            token: token,
            right: Box::new(None),
        }
    }
    pub fn unary(token: Token, node: Node) -> Self {
        Node {
            left: Box::new(None),
            token: token,
            right: Box::new(Some(node)),
        }
    }
    pub fn nodes(self) -> (Option<Node>, Option<Node>) {
        (*self.left, *self.right)
    }
}

// Just part of exercise it is not used !!
// Interpret the tree parsing it to a Reversal Polish Notation
// Example:
//   2+2*5 => [2,5,*,2,+]
//   2+2 => [2,2,+]
fn reverse_polish_notation(node: Node) -> Vec<String> {
    let token = node.clone().token;
    let mut notation = vec![];
    match node.nodes() {
        (Some(lnode), Some(rnode)) => {
            let lresult = reverse_polish_notation(lnode);
            let rresult = reverse_polish_notation(rnode);
            notation.extend(lresult.iter().cloned());
            notation.extend(rresult.iter().cloned());
            notation.push(token.value);
            return notation
        },
        (Some(lnode), None) => reverse_polish_notation(lnode),
        (None, Some(rnode)) => reverse_polish_notation(rnode),
        (None, None) => vec![token.value]
    }
}
