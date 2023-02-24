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
    pub param_ty: Type,
    pub body: Box<Expr>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct App {
    pub lambda: Box<Expr>,
    pub argm: Box<Expr>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Star {}

#[derive(Debug, Clone, PartialEq)]
pub struct KindVar {
    pub value: Symbol,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KindAbs {
    pub param: Symbol,
    pub param_ty: Box<Type>,
    pub body: Box<Kind>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetAlias {
    pub name: Symbol,
    pub value: Box<Expr>,
    pub body: Box<Expr>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAlias {
    pub name: Symbol,
    pub value: Type,
    pub body: Box<Expr>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KindAlias {
    pub name: Symbol,
    pub value: Kind,
    pub body: Box<Expr>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Anno {
    pub expr: Box<Expr>,
    pub anno: Type,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Variable {
        value: Symbol,
    },
    Product {
        param: Symbol,
        param_ty: Box<Type>,
        body: Box<Type>,
    },
    Application {
        lambda: Box<Type>,
        argm: Box<Expr>,
    },
    TyAnno {
        ty: Box<Type>,
        anno: Kind,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Star(Star),
    KindVar(KindVar),
    KindAbs(KindAbs),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(Int),
    Var(Var),
    Abs(Abs),
    App(App),
    Anno(Anno),
    LetAlias(LetAlias),
    TypeAlias(TypeAlias),
    KindAlias(KindAlias),
}

impl Expr {
    pub fn range(&self) -> Range {
        match self {
            Expr::Int(Int { range, .. }) => range.clone(),
            Expr::Var(Var { range, .. }) => range.clone(),
            Expr::Abs(Abs { range, .. }) => range.clone(),
            Expr::App(App { range, .. }) => range.clone(),
            Expr::Anno(Anno { range, .. }) => range.clone(),
            Expr::LetAlias(LetAlias { range, .. }) => range.clone(),
            Expr::TypeAlias(TypeAlias { range, .. }) => range.clone(),
            Expr::KindAlias(KindAlias { range, .. }) => range.clone(),
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

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.lambda, self.argm)
    }
}

impl fmt::Display for Star {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "*")
    }
}

impl fmt::Display for KindVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for KindAbs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Π{}: {}. {}", self.param, self.param_ty, self.body)
    }
}

impl fmt::Display for LetAlias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "let {} = {} in\n{}", self.name, self.value, self.body)
    }
}

impl fmt::Display for TypeAlias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type {}: {} in\n{}", self.name, self.value, self.body)
    }
}

impl fmt::Display for KindAlias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "kind {} = {} in\n{}", self.name, self.value, self.body)
    }
}

impl fmt::Display for Anno {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.expr, self.anno)
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::KindVar(var) => write!(f, "{}", var),
            Kind::Star(star) => write!(f, "{}", star),
            Kind::KindAbs(arrow) => write!(f, "{}", arrow),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Variable { value } => write!(f, "{}", value),
            Type::Product { param, param_ty, body } => {
                write!(f, "Π{}: {}. {}", param, param_ty, body)
            }
            Type::Application { lambda, argm } => write!(f, "{} {}", lambda, argm),
            Type::TyAnno { ty, anno } => write!(f, "{} : {}", ty, anno),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Int(int) => write!(f, "{}", int),
            Expr::Var(var) => write!(f, "{}", var),
            Expr::Abs(abs) => write!(f, "{}", abs),
            Expr::App(app) => write!(f, "{}", app),
            Expr::LetAlias(alias) => write!(f, "{}", alias),
            Expr::TypeAlias(alias) => write!(f, "{}", alias),
            Expr::KindAlias(alias) => write!(f, "{}", alias),
            Expr::Anno(anno) => write!(f, "{}", anno),
        }
    }
}
