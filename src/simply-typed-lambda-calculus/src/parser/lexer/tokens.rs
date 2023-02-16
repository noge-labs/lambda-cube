#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Variable(String),
    Number(usize),
    Lambda,
    Dot,
    LParen,
    RParen,
    Colon,
    TInt,
    Arrow,
    Error,
    Eof,
}
