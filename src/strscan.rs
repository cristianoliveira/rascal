use std::str::Chars;

#[derive(Clone)]
struct StringScanner {
    text: String,
    position: usize
}

impl StringScanner {
    pub fn new(text: String) -> Self {
        StringScanner {
            text: text,
            position: 0
        }
    }

}

impl Iterator for StringScanner {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let current = self.text.chars().nth(self.position);
        self.position += 1;
        current
    }
}

#[test]
fn it_returns_one_char_at_time() {
    let text = "abc";
    let mut string_scan = StringScanner::new(String::from(text));

    assert_eq!(string_scan.next(), Some('a'));
    assert_eq!(string_scan.next(), Some('b'));
    assert_eq!(string_scan.next(), Some('c'));
}

#[test]
fn it_returns_none() {
    let text = "a";
    let mut string_scan = StringScanner::new(String::from(text));
    let _ = string_scan.next();
    assert_eq!(string_scan.next(), None);
}

