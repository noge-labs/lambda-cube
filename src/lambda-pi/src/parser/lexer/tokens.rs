#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Number(usize),
    Variable(String),
    Lambda,

    TInt,
    Star,
    Pi,

    Dot,
    Colon,
    Equal,
    LParen,
    RParen,
    LBracket,
    RBracket,

    Error,
    Eof,
}
