
// #Kind
// Represents a type of a token
#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    // expression
    Alphanum,
    Integer,
    Operator,
    GroupBegin,
    GroupEnd,

    // Bolean
    Comparison,
    Bolean,

    // Reserved
    Begin,
    End,
    StatementEnd,
    FunctionDefine,
    FunctionParamBegin,
    FunctionParamEnd,
    ImmutableDefine,
    MutableDefine,
    Assign,
    ID,
    CONST,
    StdOut,
    Return,
    While,
    If,
    Else,

    // Others
    Separator,
    Space,
    EOF
}

impl Kind {
    // classify
    // Retrieve a Kind from a given optional char
    pub fn classify(character: &Option<char>) -> Kind {
        match *character {
            Some(value) => {
                match value {
                    ';' => Kind::StatementEnd,
                    ',' => Kind::Separator,
                    '(' => Kind::GroupBegin,
                    ')' => Kind::GroupEnd,
                    '[' => Kind::FunctionParamBegin,
                    ']' => Kind::FunctionParamEnd,
                    ' '|'\n' => Kind::Space,
                    '+'|'-'|'*'|'/'|'%' => Kind::Operator,
                    '0'|'1'|'2'|'3'|'4'|
                    '5'|'6'|'7'|'8'|'9' => Kind::Integer,
                    _ => Kind::Alphanum
                }
            },
            None => Kind::EOF
        }
    }

    // reserved
    // Retrieve a special kind for reserved keywords from a given string
    pub fn reserved(word: &String) -> Option<Kind> {
        match word.as_ref() {
            // Blocks Statements
            "fn" => Some(Kind::FunctionDefine),
            "let" | "imut" => Some(Kind::ImmutableDefine),
            "var" => Some(Kind::MutableDefine),
            "=" => Some(Kind::Assign),
            "begin" | "{" => Some(Kind::Begin),
            "end" | "}" => Some(Kind::End),
            "return" => Some(Kind::Return),

            // System
            "print" => Some(Kind::StdOut),

            // Conditionals
            "while" => Some(Kind::While),
            "if" => Some(Kind::If),
            "else" => Some(Kind::Else),
            "true"|"false" => Some(Kind::Bolean),
            "or"|"||"|"and"|"&&" => Some(Kind::Comparison),
            "=="|"!="|">"|"<" => Some(Kind::Comparison),
            _ => None
        }
    }
}

// # Token
// Represents a value and a type inside the system
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub value: String
}

impl Token {
    pub fn build(kind: Kind, value: String) -> Token {
        Token { kind: kind, value: value }
    }
}

//# Tokenizer
//
// Responsible for interpret a raw String and extract Tokens from it
#[derive(Clone)]
pub struct Tokenizer {
    pub text: String,
    pub position: usize,
    current: Option<Token>
}

impl Tokenizer {
    pub fn new(text: String) -> Self {
        Tokenizer {
            text: text,
            position: 0,
            current: None
        }
    }
}

impl Tokenizer {
    pub fn current(&self) -> Option<char> {
        self.text.chars().nth(self.position)
    }

    // next
    //
    // It store the next token from Tokenizer and return itself for
    // chaining porpouses
    pub fn advance(&mut self) -> &mut Self {
        if self.current.is_none() { self.current = self.next() }
        self
    }

    // get
    //
    // It gets the current token without consuming it
    pub fn get(&mut self) -> Option<Token> {
        self.current.clone()
    }

    pub fn peek(&mut self, next: usize) -> Option<Token> {
        let curr_position = self.position.clone();
        let next = self.next();
        self.position = curr_position;
        next
    }

    // consume
    //
    // It is responsible for consume the current Token validating the expected
    // token for the expression sintax
    pub fn consume(&mut self, expected_kind: Kind) -> Token {
        if let Some(token) = self.current.clone() {
            self.current = None;
            if token.kind != expected_kind {
                panic!(
                    "Sintax error: expected token kind {:?} found {:?} at position {}",
                    expected_kind,
                    token,
                    self.position
                    )
            }
            return token;
        } else {
            panic!("Lexer error: expected {:?} found end of file", expected_kind);
        }
    }
}
impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let current = self.current();
        let kind = Kind::classify(&current);

        self.position += 1;
        match kind {
            Kind::EOF => None,

            Kind::Space => self.next(),

            Kind::GroupBegin | Kind::GroupEnd | Kind::Operator =>
                Some(Token::build(kind, format!("{}", current.unwrap()))),

            Kind::Alphanum => {
                let mut chars = vec![current.unwrap()];
                let mut next = self.current();
                let mut kindnext = Kind::classify(&next);

                while kindnext == kind || kindnext == Kind::Integer {
                    chars.push(next.unwrap());
                    self.position += 1;
                    next = self.current();
                    kindnext = Kind::classify(&next);
                }

                let word: String = chars.clone().into_iter().collect();
                if let Some(reserved) = Kind::reserved(&word) {
                    Some(Token{ kind: reserved, value: word })
                } else {
                    Some(Token{ kind: Kind::ID, value: word })
                }
            }

            _ => {
                let mut chars = vec![current.unwrap()];
                let mut next = self.current();
                let mut kindnext = Kind::classify(&next);

                while kindnext == kind {
                    chars.push(next.unwrap());
                    self.position += 1;

                    next = self.current();
                    kindnext = Kind::classify(&next);
                }

                Some(Token::build(kind, chars.into_iter().collect()))
            }
        }
    }
}

#[test]
fn it_generate_tokens() {
    let text = "5+1";
    let mut tokens = Tokenizer::new(String::from(text));

    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Integer, value: String::from("5") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Operator, value: String::from("+") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Integer, value: String::from("1") })
    );
    assert_eq!( tokens.next(), None);
}

#[test]
fn it_ignores_empty_spaces() {
    let text = "5 + 1";
    let mut tokens = Tokenizer::new(String::from(text));

    assert_eq!(
        tokens.next().unwrap(),
        Token { kind: Kind::Integer, value: String::from("5") }
    );
    assert_eq!(
        tokens.next().unwrap(),
        Token { kind: Kind::Operator, value: String::from("+") }
    );
    assert_eq!(
        tokens.next().unwrap(),
        Token { kind: Kind::Integer, value: String::from("1") }
    );
}

#[test]
fn it_acepts_high_numbers() {
    let text = "21+1102";
    let mut tokens = Tokenizer::new(String::from(text));

    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Integer, value: String::from("21") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Operator, value: String::from("+") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Integer, value: String::from("1102") })
    );
}

#[test]
fn it_acepts_grouped_expressions() {
    let text = "(1)*1";
    let mut tokens = Tokenizer::new(String::from(text));

    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::GroupBegin, value: String::from("(") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Integer, value: String::from("1") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::GroupEnd, value: String::from(")") })
    );
}

#[test]
fn it_accepts_statements() {
    let text = "begin x = 1; end";
    let mut tokens = Tokenizer::new(String::from(text));

    assert_eq!(tokens.next(), Some(Token { kind: Kind::Begin,
                                           value: String::from("begin")}));
    assert_eq!(tokens.next(), Some(Token { kind: Kind::ID,
                                           value: String::from("x")}));
    assert_eq!(tokens.next(), Some(Token { kind: Kind::Assign,
                                           value: String::from("=")}));
    assert_eq!(tokens.next(), Some(Token { kind: Kind::Integer,
                                           value: String::from("1")}));
    assert_eq!(tokens.next(), Some(Token { kind: Kind::StatementEnd,
                                           value: String::from(";") }));
    assert_eq!(tokens.next(), Some(Token { kind: Kind::End,
                                           value: String::from("end") }));
}

#[test]
fn it_accepts_comparison_tokens() {
    let text = "true == false";
    let mut tokens = Tokenizer::new(String::from(text));

    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Bolean, value: String::from("true") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Comparison, value: String::from("==") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Bolean, value: String::from("false") })
    );
    assert_eq!( tokens.next(), None);
}

#[test]
fn it_accepts_complex_comparison_tokens() {
    let text = "true == false and true or false != false";
    let mut tokens = Tokenizer::new(String::from(text));

    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Bolean, value: String::from("true") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Comparison, value: String::from("==") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Bolean, value: String::from("false") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Comparison, value: String::from("and") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Bolean, value: String::from("true") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Comparison, value: String::from("or") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Bolean, value: String::from("false") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Comparison, value: String::from("!=") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Bolean, value: String::from("false") })
    );
}

#[test]
fn it_acepts_if_else_statements() {
    let text = "if x == y begin x = 1 else x = 2 end";
    let mut tokens = Tokenizer::new(String::from(text));

    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::If, value: String::from("if") })
    );

    tokens.next(); // x
    tokens.next(); // ==
    tokens.next(); // y
    tokens.next(); // begin
    tokens.next(); // x
    tokens.next(); // =
    tokens.next(); // 1

    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Else, value: String::from("else") })
    );
}

#[test]
fn it_acepts_function_definitions() {
    let text = "fn f = [ x, y ] { x * 2 }";
    let mut tokens = Tokenizer::new(String::from(text));
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::FunctionDefine, value: String::from("fn") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::ID, value: String::from("f") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Assign, value: String::from("=") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::FunctionParamBegin, value: String::from("[") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::ID, value: String::from("x") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Separator, value: String::from(",") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::ID, value: String::from("y") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::FunctionParamEnd, value: String::from("]") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Begin, value: String::from("{") })
    );
}

#[test]
fn it_acepts_function_calls() {
    let text = "foo(x,y);";
    let mut tokens = Tokenizer::new(String::from(text));
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::ID, value: String::from("foo") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::GroupBegin, value: String::from("(") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::ID, value: String::from("x") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Separator, value: String::from(",") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::ID, value: String::from("y") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::GroupEnd, value: String::from(")") })
    );
}

#[test]
fn it_accepts_std_output() {
    let text = "print(x+y);";
    let mut tokens = Tokenizer::new(String::from(text));
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::StdOut, value: String::from("print") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::GroupBegin, value: String::from("(") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::ID, value: String::from("x") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::Operator, value: String::from("+") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::ID, value: String::from("y") })
    );
    assert_eq!(
        tokens.next(),
        Some(Token { kind: Kind::GroupEnd, value: String::from(")") })
    );
}

