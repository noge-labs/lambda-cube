use super::location::Range;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Var {
    pub value: String,
    pub range: Range,
}

#[derive(Debug, Clone)]
pub struct Abs {
    pub param: String,
    pub body: Box<Expr>,
    pub range: Range,
}

#[derive(Debug, Clone)]
pub struct App {
    pub lambda: Box<Expr>,
    pub argm: Box<Expr>,
    pub range: Range,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Var(Var),
    Abs(Abs),
    App(App),
}

impl Expr {
    pub fn range(&self) -> Range {
        match self {
            Expr::Var(Var { range, .. }) => range.clone(),
            Expr::Abs(Abs { range, .. }) => range.clone(),
            Expr::App(App { range, .. }) => range.clone(),
        }
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for Abs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(λ{}. {})", self.param, self.body)
    }
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.lambda, self.argm)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Var(var) => write!(f, "{}", var),
            Expr::Abs(abs) => write!(f, "{}", abs),
            Expr::App(app) => write!(f, "{}", app),
        }
    }
}
