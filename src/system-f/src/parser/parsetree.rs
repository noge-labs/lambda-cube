use super::location::Range;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Int {
    pub value: usize,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    pub value: String,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Abs {
    pub param: String,
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
pub struct TAbs {
    pub param: String,
    pub body: Box<Expr>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TApp {
    pub lambda: Box<Expr>,
    pub argm: Type,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    TInt,
    TVar { value: String },
    Arrow { left: Box<Type>, right: Box<Type> },
    Forall { param: String, body: Box<Type> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(Int),
    Var(Var),
    Abs(Abs),
    App(App),
    TAbs(TAbs),
    TApp(TApp),
}

impl Expr {
    pub fn range(&self) -> Range {
        match self {
            Expr::Int(Int { range, .. }) => range.clone(),
            Expr::Var(Var { range, .. }) => range.clone(),
            Expr::Abs(Abs { range, .. }) => range.clone(),
            Expr::App(App { range, .. }) => range.clone(),
            Expr::TAbs(TAbs { range, .. }) => range.clone(),
            Expr::TApp(TApp { range, .. }) => range.clone(),
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

impl fmt::Display for TAbs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(λ{}. {})", self.param, self.body)
    }
}

impl fmt::Display for TApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} [{}])", self.lambda, self.argm)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::TInt => write!(f, "Int"),
            Type::TVar { value } => write!(f, "{}", value),
            Type::Arrow { left, right } => write!(f, "({} -> {})", left, right),
            Type::Forall { param, body } => write!(f, "∀{}. {}", param, body),
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
            Expr::TAbs(tabs) => write!(f, "{}", tabs),
            Expr::TApp(tapp) => write!(f, "{}", tapp),
        }
    }
}
