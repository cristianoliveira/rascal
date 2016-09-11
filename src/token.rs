
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

    // Statements
    Begin,
    End,
    Statement,
    StatementEnd,
    Assign,
    ID,

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
            "BEGIN" => Some(Kind::Begin),
            "END" => Some(Kind::End),
            ":=" => Some(Kind::Assign),
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
    pub position: usize
}

impl Tokenizer {
    pub fn new(text: String) -> Self {
        Tokenizer {
            text: text,
            position: 0
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let current = self.text.chars().nth(self.position);
        let kind = Kind::classify(&current);

        self.position += 1;
        match kind {
            Kind::EOF => None,

            Kind::Space => self.next(),

            Kind::Operator =>
                Some(Token::build(kind, format!("{}", current.unwrap()))),

            Kind::Alphanum => {
                let mut chars = vec![current.unwrap()];
                let mut next = self.text.chars().nth(self.position);
                let mut kindnext = Kind::classify(&next);

                while kindnext == kind || kindnext == Kind::Integer {
                    chars.push(next.unwrap());
                    self.position += 1;
                    next = self.text.chars().nth(self.position);
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
                let mut next = self.text.chars().nth(self.position);
                let mut kindnext = Kind::classify(&next);

                while kindnext == kind {
                    chars.push(next.unwrap());
                    self.position += 1;

                    next = self.text.chars().nth(self.position);
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
        Some(Token {
            kind: Kind::Integer,
            value: String::from("5")
        })
    );
    assert_eq!(
        tokens.next(),
        Some(Token {
            kind: Kind::Operator,
            value: String::from("+")
        })
    );
    assert_eq!(
        tokens.next(),
        Some(Token {
            kind: Kind::Integer,
            value: String::from("1")
        })
    );
    assert_eq!(
        tokens.next(),
        None
    );
}

#[test]
fn it_ignores_empty_spaces() {
    let text = "5 + 1";
    let mut tokens = Tokenizer::new(String::from(text));

    assert_eq!(
        tokens.next().unwrap(),
        Token {
            kind: Kind::Integer,
            value: String::from("5")
        }
    );
    assert_eq!(
        tokens.next().unwrap(),
        Token {
            kind: Kind::Operator,
            value: String::from("+")
        }
    );
    assert_eq!(
        tokens.next().unwrap(),
        Token {
            kind: Kind::Integer,
            value: String::from("1")
        }
    );
}

#[test]
fn it_acepts_high_numbers() {
    let text = "21+1102";
    let mut tokens = Tokenizer::new(String::from(text));

    assert_eq!(
        tokens.next(),
        Some(Token {
            kind: Kind::Integer,
            value: String::from("21")
        })
    );
    assert_eq!(
        tokens.next(),
        Some(Token {
            kind: Kind::Operator,
            value: String::from("+")
        })
    );
    assert_eq!(
        tokens.next(),
        Some(Token {
            kind: Kind::Integer,
            value: String::from("1102")
        })
    );
}

#[test]
fn it_acepts_grouped_expressions() {
    let text = "(1)*1";
    let mut tokens = Tokenizer::new(String::from(text));

    assert_eq!(
        tokens.next(),
        Some(Token {
            kind: Kind::GroupBegin,
            value: String::from("(")
        })
    );
    assert_eq!(
        tokens.next(),
        Some(Token {
            kind: Kind::Integer,
            value: String::from("1")
        })
    );
    assert_eq!(
        tokens.next(),
        Some(Token {
            kind: Kind::GroupEnd,
            value: String::from(")")
        })
    );
}

#[test]
fn it_accepts_statements() {
    let text = "BEGIN x := 1; END";
    let mut tokens = Tokenizer::new(String::from(text));

    assert_eq!(tokens.next(), Some(Token { kind: Kind::Begin,
                                           value: String::from("BEGIN")}));
    assert_eq!(tokens.next(), Some(Token { kind: Kind::ID,
                                           value: String::from("x")}));
    assert_eq!(tokens.next(), Some(Token { kind: Kind::Assign,
                                           value: String::from(":=")}));
    assert_eq!(tokens.next(), Some(Token { kind: Kind::Integer,
                                           value: String::from("1")}));
    assert_eq!(tokens.next(), Some(Token { kind: Kind::StatementEnd,
                                           value: String::from(";") }));
    assert_eq!(tokens.next(), Some(Token { kind: Kind::End,
                                           value: String::from("END") }));
}
