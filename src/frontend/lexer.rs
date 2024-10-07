use std::iter::Peekable;
use std::str::Chars;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Tokens {
    Identifier(String),
    Number(i32),
    Operator(String),
    Keyword(String),
    Punctuation(String),
    Newline,
}

impl PartialEq for Tokens {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Tokens::Identifier(a), Tokens::Identifier(b)) => a == b,
            (Tokens::Number(a), Tokens::Number(b)) => a == b,
            (Tokens::Operator(a), Tokens::Operator(b)) => a == b,
            (Tokens::Keyword(a), Tokens::Keyword(b)) => a == b,
            (Tokens::Punctuation(a), Tokens::Punctuation(b)) => a == b,
            (Tokens::Newline, Tokens::Newline) => true,
            _ => false,
        }
    }
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            chars: content.chars().peekable(),
        }
    }

    fn number(&mut self) -> Tokens {
        let mut number = String::new();
        while let Some(&ch) = self.chars.peek() {
            match ch {
                '0'..='9' => {
                    number.push(ch);
                    self.chars.next();
                }
                _ => break,
            }
        }
        Tokens::Number(number.parse().unwrap())
    }

    fn name(&mut self) -> String {
        let mut name = String::new();
        while let Some(&ch) = self.chars.peek() {
            match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    name.push(ch);
                    self.chars.next();
                }
                _ => break,
            }
        }
        name
    }

    fn next_token(&mut self) -> Option<Tokens> {
        while let Some(&ch) = self.chars.peek() {
            match ch {
                ' ' | '\t' => {
                    self.chars.next();
                }
                _ => break,
            }
        }

        let token = match self.chars.peek()? {
            'a'..='z' | 'A'..='Z' => {
                // Identifier or Keyword
                let name = self.name();
                match name.as_str() {
                    "print" => Tokens::Keyword("print".to_string()),
                    _ => Tokens::Identifier(name),
                }
            }
            '0'..='9' => {
                // Number
                self.number()
            }
            '+' | '-' | '*' | '/' | '=' => {
                let op = self.chars.next().unwrap();
                Tokens::Operator(op.to_string())
            }
            '(' | ')' => {
                let op = self.chars.next().unwrap();
                Tokens::Punctuation(op.to_string())
            }
            '\r' => {
                self.chars.next();
                if let Some(&'\n') = self.chars.peek() {
                    self.chars.next();
                }
                Tokens::Newline
            }
            '\n' => {
                self.chars.next();
                Tokens::Newline
            }
            _ => return None,
        };

        Some(token)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Tokens;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
