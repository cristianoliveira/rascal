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

pub fn eval_tree(node: Node) -> String {
    let token = node.clone().token;
    match node.nodes() {
        (Some(lnode), Some(rnode)) => {
            let lresult = eval_tree(lnode);
            let rresult = eval_tree(rnode);
            return binary_operation(
                lresult,
                token.value,
                rresult).to_string()
        },
        (Some(lnode), None) => eval_tree(lnode),
        (None, Some(rnode)) => eval_tree(rnode),
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
fn it_eval_tree_leaf() {
    let token = Token::build(Kind::Integer, String::from("10"));
    let leaf = Node::leaf(token);

    assert_eq!("10", eval_tree(leaf))
}

#[test]
fn it_eval_the_node_binary_operation() {
    let left = Node::leaf(Token::build(Kind::Integer, String::from("3")));
    let operator = Token::build(Kind::Operator, String::from("+"));
    let right = Node::leaf(Token::build(Kind::Integer, String::from("5")));
    let node = Node::new(Some(left), operator, Some(right));

    assert_eq!("8", eval_tree(node))
}

#[test]
fn it_eval_complex_tree() {
    let left = Node::leaf(Token::build(Kind::Integer, String::from("3")));
    let operator = Token::build(Kind::Operator, String::from("*"));
    let right = Node::leaf(Token::build(Kind::Integer, String::from("5")));
    let plusnode = Node::new(Some(left), operator, Some(right));

    let operator = Token::build(Kind::Operator, String::from("+"));
    let sumright = Node::leaf(Token::build(Kind::Integer, String::from("5")));
    let sumnode = Node::new(Some(plusnode), operator, Some(sumright));

    assert_eq!("20", eval_tree(sumnode))
}
