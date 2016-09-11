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

// eval_tree 
//
// It visits each node evaluating the binary operations returning the 
// result as String
// 
// Example:
//   The expression 3 * 5 + 5 will produce the follow tree and the result 20.
//   It evaluates the left side first and applies a post-orden calc
//
//         +-+
//         |-|
//     +---------+
//     |         |
//     v         v
//    +-+       +-+
//    |*|       |5|
//  +-----+     +-+
//  |     |
//  v     v
// ---   ---
// |3|   |5|
// +-+   +-+
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
        (None, Some(rnode)) => match token {
            Token{ kind: Kind::Operator, .. } =>
                unary_operation(token.value, eval_tree(rnode)).to_string(),
                _ => eval_tree(rnode)
        },
        (None, None) => token.value
    }
}

pub fn reverse_polish_notation(node: Node) -> Vec<String> {
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

fn unary_operation(operator: String, operand: String) -> i32 {
    let ioperand = operand.parse::<i32>().unwrap();
    match operator.as_ref() {
        "+" => ioperand,
        "-" => -(ioperand),
        _ => panic!("Sintax error: invalid unary operator {}", operator)
    }
}

// binary_operation
// Eval the binary expression for the given left, operator and right operand
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

#[test]
fn it_eval_unary_operations() {
    // 2 -- 2
    let rnode = Node::leaf(Token::build(Kind::Integer, String::from("2")));
    let negative_op = Token::build(Kind::Operator, String::from("-"));
    let unarynode = Node::unary(negative_op, rnode);

    let operator = Token::build(Kind::Operator, String::from("-"));
    let left = Node::leaf(Token::build(Kind::Integer, String::from("2")));
    let sumnode = Node::new(Some(left), operator, Some(unarynode));

    assert_eq!("4", eval_tree(sumnode))
}
