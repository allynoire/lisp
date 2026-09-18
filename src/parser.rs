use std::iter::Peekable;

use crate::lexer::{Lexer, Token};

pub struct Parser<'l> {
    lexer: Peekable<Lexer<'l>>
}

#[derive(Debug)]
pub enum Error {
    UnexpectedEOL,
    UnexpectedToken,
}

#[derive(Debug, Clone)]
pub enum Expr {
    List(Vec<Expr>),
    Symbol(String),
    Number(i32)
}


impl<'l> Parser<'l> {
    pub fn new(lexer: Lexer<'l>) -> Self {
        Self { lexer: lexer.peekable() }
    }

    fn parse_list(&mut self) -> Result<Expr, Error> {
        if self.lexer.next().ok_or(Error::UnexpectedEOL)? != Token::LParen {
            return Err(Error::UnexpectedToken)
        }

        let mut list = Vec::<Expr>::new();

        loop {
            if self.lexer.next_if_eq(&Token::RParen).is_some() {
                return Ok(Expr::List(list));
            }
            let item = self.parse().ok_or(Error::UnexpectedEOL)??;
            list.push(item);
        }
    }

    fn parse_atom(&mut self) -> Result<Expr, Error> {
        match self.lexer.next().ok_or(Error::UnexpectedEOL)? {
            Token::Ident(ident) => Ok(Expr::Symbol(ident)),
            Token::Number(num) => Ok(Expr::Number(num)),
            _ => Err(Error::UnexpectedToken)
        }
    }

    pub fn parse(&mut self) -> Option<Result<Expr, Error>> { 
        match self.lexer.peek()? {
            Token::LParen => Some(self.parse_list()),
            _ => Some(self.parse_atom())
        }
    }
}
