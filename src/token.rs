
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
    Statement,
    StatementEnd,
    Assign,
    ID,
    StdOut,
    Return,

    // Others
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
                    '(' => Kind::GroupBegin,
                    ')' => Kind::GroupEnd,
                    ' ' => Kind::Space,
                    '+'|'-'|'*'|'/'|'^' => Kind::Operator,
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
            "begin" => Some(Kind::Begin),
            "end" => Some(Kind::End),
            "=" => Some(Kind::Assign),
            "print" => Some(Kind::StdOut),
            "return" => Some(Kind::Return),
            "or"|"||"|"and"|"&&"|"=="|"!=" => Some(Kind::Comparison),
            "true"|"false" => Some(Kind::Bolean),
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
            panic!("Interpreter error: unexpected end of file");
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

            Kind::Operator =>
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
