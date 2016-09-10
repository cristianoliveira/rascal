#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Integer,
    Operator,
    GroupStart,
    GroupEnd,
    Space,
    EOF
}

impl Kind {
    pub fn classify(character: &Option<char>) -> Kind {
        match *character {
            Some(value) => {
                match value {
                    '(' => Kind::GroupStart,
                    ')' => Kind::GroupEnd,
                    ' ' => Kind::Space,
                    '0'|'1'|'2'|'3'|'4'|
                    '5'|'6'|'7'|'8'|'9' => Kind::Integer,
                    '+'|'-'|'*'|'/'|'^' => Kind::Operator,
                    _ => panic!("Character not supported {}", value)
                }
            },
            None => Kind::EOF
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub value: String
}

impl Token {
    pub fn as_integer(self) -> i32 {
        self.value.parse::<i32>().unwrap()
    }

    pub fn build(kind: Kind, value: String) -> Token {
        Token { kind: kind, value: value }
    }
}

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
            _ => {
                let mut value = vec![current.unwrap()];
                let mut next = self.text.chars().nth(self.position);
                let mut kindnext = Kind::classify(&next);

                while kindnext == kind {
                    value.push(next.unwrap());
                    self.position += 1;

                    next = self.text.chars().nth(self.position);
                    kindnext = Kind::classify(&next);
                }

                Some(Token::build(kind, value.into_iter().collect()))
            }
        }
    }
}

fn as_string(ch: char) -> String {
    let mut char_as_string = String::new();
    char_as_string.push(ch);
    char_as_string
}

fn is_operator(value: char) -> bool {
    value == '+' ||
        value == '-' ||
        value == '*' ||
        value == '/'
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
