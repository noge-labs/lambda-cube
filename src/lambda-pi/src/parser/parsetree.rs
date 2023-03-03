use super::{location::Range, symbol::Symbol};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Int {
    pub value: usize,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    pub value: Symbol,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Abs {
    pub param: Symbol,
    pub param_ty: Box<Checkable>,
    pub body: Box<Checkable>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct App {
    pub lambda: Box<Expr>,
    pub argm: Box<Checkable>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Star {
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Anno {
    pub expr: Checkable,
    pub anno: Checkable,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prod {
    pub param: Symbol,
    pub param_ty: Box<Checkable>,
    pub body: Box<Checkable>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Appl {
    pub lambda: Box<Expr>,
    pub argm: Box<Checkable>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Checkable {
    Abs(Abs),
    Inf(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(Int),
    Var(Var),
    Star(Star),
    Anno(Anno),
    Prod(Prod),
    Appl(Appl),
}

impl Checkable {
    pub fn range(&self) -> Range {
        match self {
            Checkable::Abs(Abs { range, .. }) => range.clone(),
            Checkable::Inf(expr) => (*expr.clone()).range(),
        }
    }
}

impl Expr {
    pub fn range(&self) -> Range {
        match self {
            Expr::Int(Int { range, .. }) => range.clone(),
            Expr::Var(Var { range, .. }) => range.clone(),
            Expr::Appl(Appl { range, .. }) => range.clone(),
            Expr::Anno(Anno { range, .. }) => range.clone(),
            Expr::Prod(Prod { range, .. }) => range.clone(),
            Expr::Star(Star { range, .. }) => range.clone(),
        }
    }
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for Abs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(λ{}: {}. {})", self.param, self.param_ty, self.body)
    }
}

impl fmt::Display for Appl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.lambda, self.argm)
    }
}

impl fmt::Display for Star {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "*")
    }
}

impl fmt::Display for Prod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Π{}: {}. {}", self.param, self.param_ty, self.body)
    }
}

impl fmt::Display for Anno {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.expr, self.anno)
    }
}

impl fmt::Display for Checkable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Checkable::Abs(abs) => write!(f, "{}", abs),
            Checkable::Inf(inf) => write!(f, "{}", inf),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Star(_) => write!(f, "*"),
            Expr::Int(int) => write!(f, "{}", int),
            Expr::Var(var) => write!(f, "{}", var),
            Expr::Prod(prod) => write!(f, "{}", prod),
            Expr::Appl(appl) => write!(f, "{}", appl),
            Expr::Anno(anno) => write!(f, "{}", anno),
        }
    }
}
