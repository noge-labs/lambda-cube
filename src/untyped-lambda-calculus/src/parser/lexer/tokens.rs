#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Variable(String),
    Lambda,
    Dot,
    LParen,
    RParen,
    Error,
    Eof,
}
