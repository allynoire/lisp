use std::{iter::Peekable, str::Chars};


#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Ident(String),
    Number(i32),
    Unknown,
}


pub struct Lexer<'l> {
    source: Peekable<Chars<'l>>,
    line_offset: usize,
    line_number: usize,
}

impl<'l> Lexer<'l> {
    pub fn new(source: &'l str) -> Self {
        Self {
            source: source.chars().peekable(),
            line_offset: 1,
            line_number: 1
        }
    }

    fn next_char(&mut self) -> Option<char> {
        
        while let Some(ch) = self.source.next() {
            if ch == '\n' {
                self.line_number += 1;
                self.line_offset = 1;
            }
            else {
                self.line_offset += 1;
            }
    
            if !ch.is_whitespace() {
                return Some(ch)
            }
        }
        None
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.source.peek()
    }
}

impl<'l> Iterator for Lexer<'l> {
    type Item = Token;
    
    fn next(&mut self) -> Option<Self::Item> {
        
        let token = match self.next_char()? {
            '(' => Token::LParen,
            ')' => Token::RParen,
            ch if ch.is_alphabetic() => {
                let mut s = String::from(ch);
                while self.peek_char().is_some_and(|ch| ch.is_alphanumeric()) {
                    s.push(self.next_char().unwrap())
                }
                Token::Ident(s)
            }
            ch if ch.is_numeric() => {
                let mut s = String::from(ch);
                while self.peek_char().is_some_and(|ch| ch.is_numeric()) {
                    s.push(self.next_char().unwrap())
                }
                if let Ok(num) = i32::from_str_radix(s.as_str(), 10){
                    Token::Number(num)
                } else { Token::Unknown }
            }
            _ => Token::Unknown
        };

        Some(token)
    }
    
}