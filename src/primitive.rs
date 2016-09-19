use token::{Token, Kind};
use ast::Node;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::Eq;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Rem;

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

    pub fn as_bool(&self) -> bool {
        match self.clone() {
            Type::Int(s) => (s > 0),
            Type::Bool(s) => s,
            Type::Nil => false,
            _ => panic!("Value error: type {:?} cannot be used as boolean", self)
        }
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Type) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Type) -> Ordering {
        match (self.clone(), other.clone()) {
            (Type::Bool(s), Type::Bool(o)) => s.cmp(&o),
            (Type::Int(s), Type::Int(o)) => s.cmp(&o),
            _ => panic!("Operation error: invalid comparison {:?} {:?}",
                        self,
                        other)
        }
    }
}

impl Eq for Type {}

impl Add for Type {
    type Output = Type;

    fn add(self, other: Type) -> Type {
        match (self.clone(), other.clone()) {
            (Type::Int(s), Type::Int(o)) => Type::Int(s+o),
            _ =>
                panic!("Operation error: invalid add operation between
                       {:?} and {:?}", self, other)
        }
    }
}

impl Sub for Type {
    type Output = Type;

    fn sub(self, other: Type) -> Type {
        match (self.clone(), other.clone()) {
            (Type::Int(s), Type::Int(o)) => Type::Int(s-o),
            _ =>
                panic!("Operation error: invalid add operation between
                       {:?} and {:?}", self, other)
        }
    }
}

impl Mul for Type {
    type Output = Type;

    fn mul(self, other: Type) -> Type {
        match (self.clone(), other.clone()) {
            (Type::Int(s), Type::Int(o)) => Type::Int(s*o),
            _ =>
                panic!("Operation error: invalid add operation between
                       {:?} and {:?}", self, other)
        }
    }
}

impl Div for Type {
    type Output = Type;

    fn div(self, other: Type) -> Type {
        match (self.clone(), other.clone()) {
            (Type::Int(s), Type::Int(o)) => Type::Int(s/o),
            _ =>
                panic!("Operation error: invalid add operation between
                       {:?} and {:?}", self, other)
        }
    }
}

impl Rem for Type {
    type Output = Type;

    fn rem(self, other: Type) -> Type {
        match (self.clone(), other.clone()) {
            (Type::Int(s), Type::Int(o)) => Type::Int(s%o),
            _ =>
                panic!("Operation error: invalid add operation between
                       {:?} and {:?}", self, other)
        }
    }
}
#[cfg(test)]
mod integer {
    use primitive::Type;

    #[test]
    fn it_is_true() {
        assert_eq!(true, Type::Int(0) < Type::Int(4));
        assert_eq!(true, Type::Int(4) > Type::Int(0));
        assert_eq!(true, Type::Int(1) != Type::Int(0));
        assert_eq!(true, Type::Int(4) == Type::Int(4));
        assert_eq!(true, true && Type::Int(1).as_bool());
        assert_eq!(true, false || Type::Int(1).as_bool());
    }

    #[test]
    fn it_is_false() {
        assert_eq!(false, Type::Int(4) < Type::Int(0));
        assert_eq!(false, Type::Int(0) > Type::Int(4));
        assert_eq!(false, Type::Int(0) != Type::Int(0));
        assert_eq!(false, Type::Int(1) == Type::Int(4));
        assert_eq!(false, false && Type::Int(1).as_bool());
        assert_eq!(false, false || Type::Int(0).as_bool());
    }
}

#[cfg(test)]
mod boolean {
    use primitive::Type;

    #[test]
    fn it_is_true() {
        assert_eq!(true, Type::Bool(true) != Type::Bool(false));
        assert_eq!(true, Type::Bool(true) == Type::Bool(true));
        assert_eq!(true, true && Type::Bool(true).as_bool());
        assert_eq!(true, false || Type::Bool(true).as_bool());
    }

    #[test]
    fn it_is_false() {
        assert_eq!(false, Type::Bool(false) != Type::Bool(false));
        assert_eq!(false, Type::Bool(false) == Type::Bool(true));
        assert_eq!(false, false && Type::Bool(false).as_bool());
        assert_eq!(false, false || Type::Bool(false).as_bool());
    }
}
