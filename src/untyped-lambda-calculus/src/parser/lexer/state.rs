use std::{iter::Peekable, str::Chars};

use crate::parser::lexer::tokens::Token;
use crate::parser::location::{Pos, Range};

pub struct Lexer<'a> {
    pub input: &'a str,
    pub peekable: Peekable<Chars<'a>>,
    pub start_pos: usize,
    pub current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input,
            peekable: input.chars().peekable(),
            start_pos: Default::default(),
            current_pos: Default::default(),
        }
    }

    pub fn span(&self) -> usize {
        self.current_pos
    }

    pub fn next_char(&mut self) -> Option<char> {
        match self.peekable.next() {
            Some(char) if !self.input.is_empty() => {
                self.input = &self.input[char.len_utf8()..];
                self.current_pos += char.len_utf8();
                Some(char)
            }
            _ => None,
        }
    }

    pub fn make_range(&self, start_pos: usize) -> Range {
        Range::new(
            Pos::new(start_pos as u32),
            Pos::new(self.current_pos as u32),
        )
    }

    pub fn make_token(&self, token: Token, start_pos: usize) -> (Token, Range) {
        (token, self.make_range(start_pos))
    }

    pub fn accu_while(&mut self, pred: fn(char) -> bool) -> &'a str {
        let start = self.current_pos;

        while let Some(x) = self.peekable.peek().map(|c| *c) {
            if !pred(x) {
                break;
            }

            self.current_pos += x.len_utf8();
            self.peekable.next();
        }

        let size = self.current_pos - start;
        let result = &self.input[..size];
        self.input = &self.input[size..];

        result
    }
}
