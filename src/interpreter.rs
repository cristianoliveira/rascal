use token::{Token, Kind, Tokenizer};
use ast::Node;
use parser::Parser;

// # Interpreter
//
// Represents the interpreter that is responsible for interpret 
// the Abstracted Sintax Tree generated by the Parser

// eval_tree
//
// It visits each node evaluating the binary operations returning the
// result as String
//
// Example:
//   The expression 3 * 5 + 5 will produce the follow tree and the result 20.
//   It evaluates the left side first and applies a post-order calc
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
pub fn eval_tree(tree: Node) -> String {
    let token = tree.clone().token;
    match tree.nodes() {
        (Some(lnode), Some(rnode)) => {
            return binary_operation(eval_tree(lnode),
                                    token.value,
                                    eval_tree(rnode)).to_string()
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

// unary_operation
// Resolves the unary operations Example: --1 == 1, 1++-1==0
fn unary_operation(operator: String, operand: String) -> i32 {
    let ioperand = if let Ok(val) = operand.parse::<i32>() { val } else {
        panic!("Sintax error: invalid unary operand {}", operand)
    };
    match operator.as_ref() {
        "+" => ioperand,
        "-" => -(ioperand),
        _ => panic!("Sintax error: invalid unary operator {}", operator)
    }
}

// binary_operation
// Resolve binary expression for the given left, operator and right operand
fn binary_operation(operand: String, operator: String, operand2: String) -> i32 {
    let left = if let Ok(val) = operand.parse::<i32>() { val } else {
        panic!("Sintax error: invalid operand: {}", operand)
    };
    let right = if let Ok(val) = operand2.parse::<i32>() { val } else {
        panic!("Sintax error: invalid operand: {}", operand2)
    };
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
    // 3+5
    let left = Node::leaf(Token::build(Kind::Integer, String::from("3")));
    let operator = Token::build(Kind::Operator, String::from("+"));
    let right = Node::leaf(Token::build(Kind::Integer, String::from("5")));
    let node = Node::new(Some(left), operator, Some(right));

    assert_eq!("8", eval_tree(node))
}

#[test]
fn it_eval_complex_tree() {
    // 5+5*3
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

#[test]
fn it_sums() {
    let text = "5+1";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    assert_eq!("6", eval_tree(parser.parse()));
}

#[test]
fn it_substract() {
    let text = "5-1";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    assert_eq!("4", eval_tree(parser.parse()));
}

#[test]
fn it_multiplies() {
    let text = "5*2";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    assert_eq!("10", eval_tree(parser.parse()));
}

#[test]
fn it_divide() {
    let text = "4/2";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    assert_eq!("2", eval_tree(parser.parse()));
}

#[test]
fn it_accepts_multiples_operation() {
    let text = "10+5-4-1";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    assert_eq!("10", eval_tree(parser.parse()));
}

#[test]
fn it_respect_precedence() {
    let text = "1+1*2";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    assert_eq!("3", eval_tree(parser.parse()));
}

#[test]
fn it_respects_grouped_expression() {
    let text = "4+(1+(1+1)*2)+1";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    assert_eq!("10", eval_tree(parser.parse()));
}

#[test]
fn it_accept_unary_operations() {
    let text = "(4+-1)--2";
    let tokenizer = Tokenizer::new(String::from(text));
    let mut parser = Parser::new(tokenizer);

    assert_eq!("5", eval_tree(parser.parse()));
}
