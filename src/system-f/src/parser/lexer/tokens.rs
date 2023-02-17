#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Number(usize),
    Variable(String),
    Lambda,
    Dot,
    Colon,

    TInt,
    TVar(String),
    Arrow,
    Forall,

    LParen,
    RParen,
    LBracket,
    RBracket,

    Error,
    Eof,
}
