use core::fmt;
use std::fmt::Display;

use crate::parser::parsetree::Expr;

#[derive(Debug)]
pub enum TypeError {
    Mismatch(Expr, Expr),
    UndefinedVariable(String),
    UnexpectedType(Expr),
    VariableClash,
    TypeClash,
}

impl Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeError::Mismatch(e, r) => write!(f, "expect {} but got {}", e, r),
            TypeError::UndefinedVariable(v) => write!(f, "unbounded variable {}", v),
            TypeError::UnexpectedType(t) => write!(f, "unexpected type {}", t),
            TypeError::VariableClash => write!(f, "variable clash"),
            TypeError::TypeClash => write!(f, "type clash"),
        }
    }
}
