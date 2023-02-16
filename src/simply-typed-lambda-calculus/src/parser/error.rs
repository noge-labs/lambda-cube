use std::fmt;

use super::lexer::tokens::Token;
use super::location::Range;

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(Token, Range),
    UnexpectedEOF,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken(token, _) => {
                write!(f, "UnexpectedToken {:?}, todo: error message", token)
            }
            ParserError::UnexpectedEOF => write!(f, "UnexpectedEof, todo: error message"),
        }
    }
}
