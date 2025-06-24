use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
    EOF,
    Whitespace,
    Bad,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Number(_) => write!(f, "Number"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::EOF => write!(f, "EOF"),
            TokenKind::Whitespace => write!(f, "Whitespace"),
            TokenKind::Bad => write!(f, "Bad"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextSpan {
    start: usize,
    end: usize,
    pub(crate) literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self { start, end, literal }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: TextSpan
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl <'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, current_pos: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos > self.input.len() {
            return None
        }
        if self.current_pos == self.input.len() {
            self.current_pos += 1;
            let eof_char = '\0';
            let span = TextSpan::new(0,0, eof_char.to_string());
            return Some(Token::new(TokenKind::EOF, span));
        }

        let start = self.current_pos;
        let current_char = self.get_current_char();
        current_char.map(|c| {
            let mut kind = TokenKind::Bad;
            if Self::is_number_start(&c) {
                let number: i64 = self.consume_number();
                kind = TokenKind::Number(number);
            } else if Self::is_whitespace(&c) {
                self.consume_char();
                kind = TokenKind::Whitespace;
            }
            else {
                kind = self.consume_punctuation();
            }

            let end = self.current_pos;
            let literal = self.input[start..end].to_string();
            let span = TextSpan::new(start, end, literal);
            Token::new(kind, span)
        })
    }

    fn is_number_start(current_char: &char) -> bool {
        current_char.is_ascii_digit()
    }

    fn is_whitespace(current_char: &char) -> bool {
        current_char.is_whitespace()
    }

    fn get_current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    fn consume_char(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None
        }
        let current_char = self.get_current_char();
        self.current_pos += 1;

        current_char
    }

    fn consume_number(&mut self) -> i64 {
        let mut number = 0;
        while let Some(c) = self.get_current_char() {
            if c.is_digit(10) {
                self.consume_char();
                number = number * 10 + (c.to_digit(10).unwrap() as i64);
            } else {
                break;
            }
        }
        number
    }

    fn consume_punctuation(&mut self) -> TokenKind {
        let c = self.consume_char().unwrap();
        match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            _ => TokenKind::Bad
        }
    }
}