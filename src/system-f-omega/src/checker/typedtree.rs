use std::fmt;

use crate::parser::symbol::Symbol;

#[derive(Debug, Clone, PartialEq)]
pub struct Annoted {
    pub desc: Box<Type>,
    pub kind: Kind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Star,
    KindArrow { left: Box<Kind>, right: Box<Kind> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Var {
        value: Symbol,
    },
    Arrow {
        left: Annoted,
        right: Annoted,
    },
    Forall {
        param: Symbol,
        param_ty: Kind,
        body: Annoted,
    },
    TyAbs {
        param: Symbol,
        param_ty: Kind,
        body: Annoted,
    },
    TyApp {
        lambda: Annoted,
        argm: Annoted,
    },
}

impl fmt::Display for Annoted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Annoted { desc, .. } => write!(f, "{}", desc),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Var { value } => write!(f, "{}", value),
            Type::Arrow { left, right } => write!(f, "({} -> {})", left, right),
            Type::Forall { param, param_ty, body } => {
                write!(f, "∀{}: {}. {}", param, param_ty, body)
            }
            Type::TyAbs { param, param_ty, body } => {
                write!(f, "{}: {}. {}", param, param_ty, body)
            }
            Type::TyApp { lambda, argm } => write!(f, "({} {})", lambda, argm),
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::Star => write!(f, "*"),
            Kind::KindArrow { left, right } => write!(f, "({} -> {})", left, right),
        }
    }
}
