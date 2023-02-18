use core::fmt;
use std::fmt::Display;

use crate::parser::parsetree::Type;

#[derive(Debug)]
pub enum TypeError {
    Mismatch(Type, Type),
    UndefinedVariable(String),
    UnexpectedType(Type),
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
