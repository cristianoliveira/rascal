use token::{Token, Kind};
use ast::Node;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Str(String),
    Int(i32),
    Bool(bool),
    Func(Vec<Node>, Node),
    Nil
}
impl Type {
    pub fn from(token: &Token) -> Type {
        match token.clone() {
            Token{kind: Kind::Integer, value} => {
                Type::Int(value.parse::<i32>().expect("Invalid integer value."))
            },
            Token{kind: Kind::Bolean, value} =>
                Type::Bool(value=="true"),
            _ => Type::Nil
        }
    }
    pub fn to_string(self) -> String {
        match self {
            Type::Func(_,_) => format!("function"),
            Type::Str(s) => format!("{}", s),
            Type::Int(s) => format!("{}", s),
            Type::Bool(s) => format!("{}", s),
            _ => String::new()
        }
    }
}

