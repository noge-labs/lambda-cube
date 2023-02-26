#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Number(usize),
    Variable(String),
    Lambda,
    Let,
    Fst,
    Snd,
    In,

    TInt,
    TVar(String),
    Arrow,
    Forall,
    Prod,

    Dot,
    Colon,
    Comma,
    Equal,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    Error,
    Eof,
}
