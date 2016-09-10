// This module contains the Abstract Sintax Tree representations

use token::{Token, binary_operation}

trait Node {
    fn value(self) -> Token;
    fn is_leaf(self) -> bool;
};

struct BinaryOperation{
    pub left: Option<Node>;
    pub token: Token;
    pub right: Option<Node>;
}

impl BinaryOperation {
    pub fn new(left: Option<Node>, token: Token, right: Option<Node>)
}

impl Node for BinaryOperation {
    fn value(self) -> Token {
        if let Some(lnode) = self.left {
            let mut lresult = lnode.value()
            if let Some(rnode) = self.right {
                let lresult = rnode.value()
                lresult.value = token::binary_operation(&lresult.clone(),
                                                        &self.value,
                                                        &rresult)
            };
            return lresult;
        } else {
            panic!("AST error: invalid binary operation node")
        }
    }
    fn is_leaf(self) -> bool {
        self.left.is_none && self.right.is_none()
    }
}

struct Number{
    pub token: Token;
}
impl Node for Number {
    fn value(self) -> Token {
        self.token
    }
    fn is_leaf(self) -> bool {
        true
    }
}

pub fn visit(node: Node) -> String {
    if node.is_leaf() { return node.value() }
    if let Some(lnode) = node.left() {
        let lresult = lnode.value();
        if Some(rnode) = node.right() {
            let rresult = rnode.value();
            let operator = node.value();
            return binary_operation(
                &lresult,
                &operator,
                &rresult).to_string()
        }
    }
}
