use core::fmt;
use std::fmt::Display;

use crate::parser::parsetree::Type;

use super::typedtree as T;

#[derive(Debug)]
pub enum TypeError {
    Mismatch(Type, Type),
    UndefinedVariable(String),
    UnexpectedType(Type),
    VariableClash,
    TypeClash,
    TypeNotAForall(T::Type),
    TypeNotAArrow(T::Type),
}

impl Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeError::Mismatch(e, r) => write!(f, "expect {} but got {}", e, r),
            TypeError::UndefinedVariable(v) => write!(f, "unbounded variable {}", v),
            TypeError::UnexpectedType(t) => write!(f, "unexpected type {}", t),
            TypeError::VariableClash => write!(f, "variable clash"),
            TypeError::TypeClash => write!(f, "type clash"),
            TypeError::TypeNotAForall(t) => write!(f, "TypeNotAForall {}", t),
            TypeError::TypeNotAArrow(t) => write!(f, "TypeNotAArrow {}", t),
        }
    }
}
