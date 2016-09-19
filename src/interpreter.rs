use ast::{Node, Operation};
use primitive::Type;
use frame::{Frame, FrameStack};

// # Interpreter
//
// Represents the interpreter that is responsible for interpret 
// the Abstracted Sintax Tree generated by the Parser

pub struct Interpreter {
    pub stack: FrameStack,
}
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            stack: FrameStack::new()
        }
    }

    fn scope(&mut self) -> &mut Frame {
        self.stack.current()
    }

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
    pub fn eval(&mut self, tree: Node) -> String {
        self.eval_tree(tree).to_string()
    }

    pub fn eval_tree(&mut self, tree: Node) -> Type {
        let Node{operation, ..} = tree;
        match *operation.clone() {
            Operation::IfElse(conditional, lnode, rnode) => {
                let condition = self.eval_tree(conditional);

                if truthy(condition) {
                    self.eval_tree(lnode)
                } else {
                    self.eval_tree(rnode)
                }
            },

            Operation::Binary(lnode, operator, rnode) => {
                binary_operation(
                    self.eval_tree(lnode), operator, self.eval_tree(rnode)
                )
            },

            Operation::Comparison(lnode, operator, rnode) =>
                binary_comparison(
                    self.eval_tree(lnode), operator, self.eval_tree(rnode)),

            Operation::CallFunc(nodename, params) => {
                let func_frame = self.scope().clone();
                self.stack.push(func_frame);

                let name = nodename.value;
                if let Type::Func(fparams, block) = self.scope().functions.get(&*name).unwrap().clone() {
                    for (pname, pvalue) in fparams.iter().zip(params.iter()) {
                        let value = self.eval_tree(pvalue.clone());
                        self.scope().locals.insert(pname.clone().value, value);
                    }

                    self.eval_tree(block)
                } else {
                    panic!("Value error: {} is not callable", name)
                }
            },

            Operation::DefineFunc(lnode, func) => {
                let name = lnode.value;
                if self.scope().has(&*name) {
                    panic!("Value error: variable {} has already defined.", name)
                }

                self.scope().functions.insert(name, func);
                Type::Nil
            },

            Operation::DefineImut(lnode, rnode) => {
                let name = lnode.value;
                let value = self.eval_tree(rnode);

                if self.scope().has(&*name) {
                    panic!("Value error: variable {} has already defined.", name)
                }

                self.scope().ilocals.insert(name, value.clone());
                Type::Nil
            },

            Operation::DefineVar(lnode, rnode) => {
                let name = lnode.value;
                let value = self.eval_tree(rnode);

                if self.scope().has(&*name) {
                    panic!("Value error: variable {} has already defined.", name)
                }

                self.scope().locals.insert(name, value.clone());
                value
            },

            Operation::ReAssign(lnode, rnode) => {
                let name = lnode.value;
                let value = self.eval_tree(rnode);

                if !self.scope().has(&*name) {
                    panic!("Value error: variable {} used before declared.", name)
                }

                if self.scope().is_imutable(&*name) {
                    panic!("Value error: imutable {} was reassigned.", name)
                }

                self.scope().locals.insert(name, value.clone());
                Type::Nil
            },

            Operation::NegUnary(node) => {
                unary_operation("-", self.eval_tree(node))
            },

            Operation::Return(node) => self.eval_tree(node),

            Operation::Block(statements) => {
                let copy_scope = self.scope().clone();
                self.stack.push(copy_scope);
                let mut last_stm_return = Type::Nil;
                for stm in statements {
                    last_stm_return = self.eval_tree(stm.clone())
                }
                self.stack.pop();
                last_stm_return
            },

            Operation::Loop(conditional, block) => {
                let mut condition = self.eval_tree(conditional.clone());

                while truthy(condition) {
                    let _ = self.eval_tree(block.clone());
                    condition = self.eval_tree(conditional.clone());
                }

                return Type::Nil
            },

            Operation::Identifier(name) => self.scope().get(&*name),

            Operation::Constant(var) => var,

            _ => Type::Nil
        }
    }
}

// unary_operation
// Resolves the unary operations Example: --1 == 1, 1++-1==0
fn unary_operation(operator: &str, operand: Type) -> Type {
    let result = match (operator , operand.clone()) {
        ("+", Type::Int(val)) => val,
        ("-", Type::Int(val)) => -val,
        _ => panic!("Operation error: invalid operation {:?}{:?}",
                    operator, operand)
    };
    Type::Int(result)
}

// binary_operation
// Resolve binary expression for the given left, operator and right operand
fn binary_operation(left: Type, operator: String, right: Type) -> Type {
    let operleft = if let Ok(val) = left.clone().to_string().parse::<i32>() { val } else {
        panic!("Sintax error: invalid operand: {:?}", left)
    };
    let operright = if let Ok(val) = right.clone().to_string().parse::<i32>() { val } else {
        panic!("Sintax error: invalid operand: {:?}", right)
    };
    let result = match operator.as_ref() {
        "+" => operleft + operright,
        "-" => operleft - operright,
        "*" => operleft * operright,
        "/" => operleft / operright,
        _ => panic!("Sintax error: invalid operator {:?}", operator)
    };

    Type::Int(result)
}

// binary_operation
// Resolve binary expression for the given left, operator and right operand
fn binary_comparison(left: Type, operator: String, right: Type) -> Type {
    let bleft = left.clone().to_string().replace("true","1").replace("false","0");
    let bright = right.clone().to_string().replace("true","1").replace("false","0");

    let operleft = if let Ok(val) = bleft.parse::<i32>() { val } else {
        panic!("Sintax error: invalid operand: {:?}", left)
    };
    let operright = if let Ok(val) = bright.parse::<i32>() { val } else {
        panic!("Sintax error: invalid operand: {:?}", right)
    };

    let result = match operator.as_ref() {
        "==" => operleft == operright,
        "!=" => operleft != operright,
        "<" => operleft < operright,
        ">" => operleft > operright,
        "or"|"||" => operleft == 1 || operright == 1,
        "and"|"&&" => operleft == 1 && operright == 1,
        _ => panic!("Sintax error: invalid operator {:?}", operator)
    };

    Type::Bool(result)
}

fn truthy(condition: Type) -> bool {
    binary_comparison(
        condition,
        String::from("=="),
        Type::Bool(true)).to_string() == "true"
}


#[cfg(test)]
mod interpreter {

    use token::{Token, Kind, Tokenizer};
    use interpreter::Interpreter;
    use parser::Parser;
    use ast::Node;

    #[test]
    fn it_eval_tree_leaf() {
        let token = Token::build(Kind::Integer, String::from("10"));
        let leaf = Node::constant(token);

        assert_eq!("10", Interpreter::new().eval(leaf))
    }

    #[test]
    fn it_eval_the_node_binary_operation() {
        // 3+5
        let left = Node::constant(Token::build(Kind::Integer, String::from("3")));
        let operator = Token::build(Kind::Operator, String::from("+"));
        let right = Node::constant(Token::build(Kind::Integer, String::from("5")));
        let node = Node::binary(left, operator, right);

        assert_eq!("8", Interpreter::new().eval(node))
    }

    #[test]
    fn it_eval_complex_tree() {
        // 5+5*3
        let left = Node::constant(Token::build(Kind::Integer, String::from("3")));
        let operator = Token::build(Kind::Operator, String::from("*"));
        let right = Node::constant(Token::build(Kind::Integer, String::from("5")));
        let plusnode = Node::binary(left, operator, right);

        let operator = Token::build(Kind::Operator, String::from("+"));
        let sumright = Node::constant(Token::build(Kind::Integer, String::from("5")));
        let sumnode = Node::binary(plusnode, operator, sumright);

        assert_eq!("20", Interpreter::new().eval(sumnode))
    }

    #[test]
    fn it_eval_unary_operations() {
        // 2 -- 2
        let rnode = Node::constant(Token::build(Kind::Integer, String::from("2")));
        let negative_op = Token::build(Kind::Operator, String::from("-"));
        let unarynode = Node::unary(negative_op, rnode);

        let operator = Token::build(Kind::Operator, String::from("-"));
        let left = Node::constant(Token::build(Kind::Integer, String::from("2")));
        let sumnode = Node::binary(left, operator, unarynode);

        assert_eq!("4", Interpreter::new().eval(sumnode))
    }

    #[test]
    fn it_sums() {
        let text = "5+1";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        assert_eq!("6", Interpreter::new().eval(parser.parse()));
    }

    #[test]
    fn it_substract() {
        let text = "5-1";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        assert_eq!("4", Interpreter::new().eval(parser.parse()));
    }

    #[test]
    fn it_multiplies() {
        let text = "5*2";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        assert_eq!("10", Interpreter::new().eval(parser.parse()));
    }

    #[test]
    fn it_divide() {
        let text = "4/2";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        assert_eq!("2", Interpreter::new().eval(parser.parse()));
    }

    #[test]
    fn it_accepts_multiples_operation() {
        let text = "10+5-4-1";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        assert_eq!("10", Interpreter::new().eval(parser.parse()));
    }

    #[test]
    fn it_respect_precedence() {
        let text = "1+1*2";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        assert_eq!("3", Interpreter::new().eval(parser.parse()));
    }

    #[test]
    fn it_respects_grouped_expression() {
        let text = "4+(1+(1+1)*2)+1";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        assert_eq!("10", Interpreter::new().eval(parser.parse()));
    }

    #[test]
    fn it_accept_unary_operations() {
        let text = "(4+-1)--2";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        assert_eq!("5", Interpreter::new().eval(parser.parse()));
    }

    #[test]
    fn it_accept_binary_comparison() {
        let text = "4 == 2";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        assert_eq!("false", Interpreter::new().eval(parser.parse()));
    }

    #[test]
    fn it_accept_composed_binary_comparison() {
        let text = "1 > 1 or 2 == 2 and 3 != 3";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);

        assert_eq!("false", Interpreter::new().eval(parser.parse()));
    }


    #[test]
    fn it_eval_block_assigning_vars_to_symbol_table() {
        let text = "begin mut x = 10; return x end";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);
        let mut interpreter = Interpreter::new();
        let result = interpreter.eval(parser.parse());
        assert_eq!("10", result);
    }

    #[test]
    fn it_eval_block_retrieve_vars_from_symbol_table() {
        let text = "begin imut x = 10; mut y = x + 5; return y end";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);
        let mut interpreter = Interpreter::new();
        let result = interpreter.eval(parser.parse());

        assert_eq!("15", result);
    }

    #[test]
    fn it_eval_functions_without_params() {
        let text = "{ fun two = [] { return 2 }; two() }";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);
        let mut interpreter = Interpreter::new();
        let result = interpreter.eval(parser.parse());

        assert_eq!("2", result);
    }

    #[test]
    fn it_eval_functions_with_params() {
        let text = "{ fun add = [x] { return x + 2 }; add(2) }";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);
        let mut interpreter = Interpreter::new();
        let result = interpreter.eval(parser.parse());

        assert_eq!("4", result);
    }

    #[test]
    fn it_eval_functions_with_multiple_params() {
        let text = "{ fun add = [x,y,z] { return x + y + z }; add(2,1,2) }";
        let tokenizer = Tokenizer::new(String::from(text));
        let mut parser = Parser::new(tokenizer);
        let mut interpreter = Interpreter::new();
        let result = interpreter.eval(parser.parse());

        assert_eq!("5", result);
    }

}
