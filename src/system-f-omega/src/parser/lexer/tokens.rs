#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Number(usize),
    Variable(String),
    Lambda,
    Kind,
    Type,
    Let,
    In,

    TInt,
    TVar(String),
    Arrow,
    Forall,

    Star,

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
