// This module contains the Abstract Sintax Tree representations

use token::{Token, Kind};

#[derive(Debug, Clone, PartialEq)]
pub struct Node{
    pub token: Token,
    left: Box<Option<Node>>,
    right: Box<Option<Node>>
}

impl Node {
    pub fn new(left: Option<Node>, token: Token, right: Option<Node>) -> Self {
        Node {
            left: Box::new(left),
            token: token,
            right: Box::new(right)
        }
    }
    pub fn leaf(token: Token) -> Self {
        Node {
            left: Box::new(None),
            token: token,
            right: Box::new(None)
        }
    }
    pub fn nodes(self) -> (Option<Node>, Option<Node>) {
        (*self.left, *self.right)
    }
}

pub fn visit(node: Node) -> String {
    let token = node.clone().token;
    match node.nodes() {
        (Some(lnode), Some(rnode)) => {
            let lresult = visit(lnode);
            let rresult = visit(rnode);
            return binary_operation(
                lresult,
                token.value,
                rresult).to_string()
        },
        (Some(lnode), None) => visit(lnode),
        (None, Some(rnode)) => visit(rnode),
        (None, None) => token.value
    }
}

fn binary_operation(operand: String, operator: String, operand2: String) -> i32 {
    let left = operand.parse::<i32>().unwrap();
    let right = operand2.parse::<i32>().unwrap();
    match operator.as_ref() {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => panic!("Sintax error: invalid operator {}", operator)
    }
}

#[test]
fn it_visits_leaf() {
    let token = Token::build(Kind::Integer, String::from("10"));
    let leaf = Node::leaf(token);

    assert_eq!("10", visit(leaf))
}
