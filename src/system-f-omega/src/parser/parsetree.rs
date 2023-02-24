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
pub struct TAbs {
    pub param: Symbol,
    pub param_ty: Kind,
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
pub struct Anno {
    pub expr: Box<Expr>,
    pub anno: Type,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TInt {}

#[derive(Debug, Clone, PartialEq)]
pub struct TVar {
    pub value: Symbol,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Forall {
    pub param: Symbol,
    pub param_ty: Kind,
    pub body: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arrow {
    pub left: Box<Type>,
    pub right: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TyAbs {
    pub param: Symbol,
    pub param_ty: Kind,
    pub body: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TyApp {
    pub lambda: Box<Type>,
    pub argm: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TyAnno {
    pub ty: Box<Type>,
    pub anno: Kind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Star {}

#[derive(Debug, Clone, PartialEq)]
pub struct KindVar {
    pub value: Symbol,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KindArrow {
    pub left: Box<Kind>,
    pub right: Box<Kind>,
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
pub enum Kind {
    Star(Star),
    KindVar(KindVar),
    KindArrow(KindArrow),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    TInt(TInt),
    TVar(TVar),
    Arrow(Arrow),
    Forall(Forall),
    TyAbs(TyAbs),
    TyApp(TyApp),
    TyAnno(TyAnno),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(Int),
    Var(Var),
    Abs(Abs),
    App(App),
    TAbs(TAbs),
    TApp(TApp),
    LetAlias(LetAlias),
    TypeAlias(TypeAlias),
    KindAlias(KindAlias),
    Anno(Anno),
}

impl Expr {
    pub fn range(&self) -> Range {
        match self {
            Expr::Anno(Anno { range, .. }) => range.clone(),
            Expr::Int(Int { range, .. }) => range.clone(),
            Expr::Var(Var { range, .. }) => range.clone(),
            Expr::Abs(Abs { range, .. }) => range.clone(),
            Expr::App(App { range, .. }) => range.clone(),
            Expr::TAbs(TAbs { range, .. }) => range.clone(),
            Expr::TApp(TApp { range, .. }) => range.clone(),
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

impl fmt::Display for TAbs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(λ{}: {}. {})", self.param, self.param_ty, self.body)
    }
}

impl fmt::Display for TApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} [{}])", self.lambda, self.argm)
    }
}

impl fmt::Display for Arrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.left {
            Type::Arrow(_) => write!(f, "({}) -> {}", self.left, self.right),
            _ => write!(f, "{} -> {}", self.left, self.right),
        }
    }
}

impl fmt::Display for TVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for Forall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(∀{}: {}. {})", self.param, self.param_ty, self.body)
    }
}

impl fmt::Display for TyAbs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(λ{}: {}. {})", self.param, self.param_ty, self.body)
    }
}

impl fmt::Display for TyApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.lambda, self.argm)
    }
}

impl fmt::Display for TyAnno {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.ty, self.anno)
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

impl fmt::Display for KindArrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.left {
            Kind::KindArrow(_) => write!(f, "({}) -> {}", self.left, self.right),
            _ => write!(f, "{} -> {}", self.left, self.right),
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kind::KindVar(var) => write!(f, "{}", var),
            Kind::Star(star) => write!(f, "{}", star),
            Kind::KindArrow(arrow) => write!(f, "{}", arrow),
        }
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

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::TInt(_) => write!(f, "Int"),
            Type::TVar(tvar) => write!(f, "{}", tvar),
            Type::Arrow(arrow) => write!(f, "{}", arrow),
            Type::Forall(forall) => write!(f, "{}", forall),
            Type::TyAbs(tyabs) => write!(f, "{}", tyabs),
            Type::TyApp(tyapp) => write!(f, "{}", tyapp),
            Type::TyAnno(anno) => write!(f, "{}", anno),
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
            Expr::LetAlias(alias) => write!(f, "{}", alias),
            Expr::TypeAlias(alias) => write!(f, "{}", alias),
            Expr::KindAlias(alias) => write!(f, "{}", alias),
            Expr::Anno(anno) => write!(f, "{}", anno),
        }
    }
}
