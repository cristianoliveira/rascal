// This module contains the Abstract Sintax Tree representations

use token::{Token};
use primitive::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Identifier(String),
    Constant(Type),
    Binary(Node, String, Node),
    Comparison(Node, String, Node),
    CallFunc(Node, Vec<Node>),
    DefineFunc(Node, Type),
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
    pub value: String,
}

impl Node {
    pub fn binary(left: Node, token: String, right: Node) -> Self {
        Node {
            operation: Box::new(Operation::Binary(left, token.clone(), right)),
            value: token,
        }
    }
    pub fn comparison(left: Node, token: String, right: Node) -> Self {
        Node {
            operation: Box::new(Operation::Comparison(left, token.clone(), right)),
            value: token,
        }
    }
    pub fn call_function(id: Node, params: Vec<Node>) -> Self {
        Node {
            operation: Box::new(
                Operation::CallFunc(id, params)
                ),
            value: String::from("=")
        }
    }
    pub fn define_function(id: Node, params: Vec<Node>, block: Node) -> Self {
        Node {
            operation: Box::new(
                Operation::DefineFunc(id, Type::Func(params, block))
                ),
            value: String::from("=")
        }
    }
    pub fn define_immutable(left: Node, right: Node) -> Self {
        Node {
            operation: Box::new(Operation::DefineImut(left, right)),
            value: String::from("=")
        }
    }
    pub fn define_mutable(left: Node, right: Node) -> Self {
        Node {
            operation: Box::new(Operation::DefineVar(left, right)),
            value: String::from("=")
        }
    }
    pub fn reassign(left: Node, right: Node) -> Self {
        Node {
            operation: Box::new(Operation::ReAssign(left, right)),
            value: String::from("")
        }
    }
    pub fn indentifier(token: Token) -> Self {
        Node {
            operation: Box::new(Operation::Identifier(token.clone().value)),
            value: token.value
        }
    }
    pub fn constant(token: Token) -> Self {
        let primitive = Type::from(&token);
        Node {
            operation: Box::new(Operation::Constant(primitive)),
            value: token.value
        }
    }
    pub fn unary(token: Token, node: Node) -> Self {
        Node {
            operation: Box::new(Operation::NegUnary(node)),
            value: token.value
        }
    }
    pub fn _return(node: Node) -> Self {
        Node {
            operation: Box::new(Operation::Return(node)),
            value: String::new()
        }
    }
    pub fn ifelse(condition: Node, if_node: Node, else_node: Node) -> Self {
        Node {
            operation: Box::new(Operation::IfElse(condition, if_node, else_node)),
            value: String::new()
        }
    }
    pub fn conditional(node:Node, statements: Node) -> Self {
        Node {
            operation: Box::new(Operation::Loop(node, statements)),
            value: String::new()
        }
    }
    pub fn block(statements: Vec<Node>) -> Self {
        Node {
            operation: Box::new(Operation::Block(statements)),
            value: String::new()
        }
    }
    pub fn empty() -> Self {
        Node {
            operation: Box::new(Operation::Empty),
            value: String::new()
        }
    }
}
