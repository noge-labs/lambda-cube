use crate::parser::{error::ParserError, lexer::tokens::Token, location::Range, Lexer};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: (Token, Range),
    next_token: (Token, Range),
    after: usize,
}

impl<'a> Parser<'a> {
    pub fn init(file: &'a mut str) -> Result<Parser<'a>, ParserError> {
        let mut lexer = Lexer::new(file);
        let current_token = lexer.lex_token();
        let next_token = lexer.lex_token();

        Ok(Parser {
            lexer,
            current_token,
            next_token,
            after: Default::default(),
        })
    }

    pub fn get(&self) -> &Token {
        &self.current_token.0
    }

    pub fn get_next(&self) -> &Token {
        &self.next_token.0
    }

    pub fn try_single<T>(
        &mut self,
        f: fn(&mut Parser<'a>) -> Result<T, ParserError>,
    ) -> Result<Option<T>, ParserError> {
        let current = self.after;

        match f(self) {
            Ok(res) => Ok(Some(res)),
            Err(_) if current == self.after => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub fn advance(&mut self) -> Result<(Token, Range), ParserError> {
        let current = self.current_token.clone();

        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.lex_token();
        self.after += 1;

        Ok(current)
    }

    pub fn consume<T>(
        &mut self,
        expect: fn(&Token) -> Option<T>,
    ) -> Result<(T, Range), ParserError> {
        match expect(self.get()) {
            None => self.fail(),
            Some(res) => {
                let range = self.current_token.1.clone();
                self.advance()?;
                Ok((res, range))
            }
        }
    }

    pub fn fail<T>(&mut self) -> Result<T, ParserError> {
        let (token, range) = self.current_token.clone();
        Err(ParserError::UnexpectedToken(token, range))
    }
}
