#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Number(usize),
    Variable(String),
    Lambda,
    Let,
    In,

    TInt,
    TVar(String),
    Arrow,
    Forall,

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
