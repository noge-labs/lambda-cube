use crate::parser::parsetree::{Abs, App, Expr, Int, TAbs, TApp, Type, Var};
use std::collections::HashSet;

pub fn free_variables(expr: Expr) -> HashSet<String> {
    let mut free: HashSet<String> = HashSet::new();

    match expr {
        Expr::Int(Int { .. }) => (),
        Expr::Var(Var { value, .. }) => {
            free.insert(value);
        }
        Expr::App(App { lambda, argm, .. }) => {
            free.extend(free_variables(*lambda));
            free.extend(free_variables(*argm));
        }
        Expr::Abs(Abs { param, body, .. }) => {
            free.extend(free_variables(*body));
            free.remove(&param);
        }
        Expr::TAbs(TAbs { body, .. }) => free.extend(free_variables(*body)),
        Expr::TApp(TApp { lambda, .. }) => free.extend(free_variables(*lambda)),
    }

    free
}

pub fn substitution(expr: Expr, from: String, to: Expr) -> Expr {
    match expr.clone() {
        Expr::Var(Var { value, .. }) if value == from => to,
        Expr::Var(Var { .. }) => expr,
        Expr::Int(Int { .. }) => expr,
        Expr::App(app) => {
            let lambda = substitution(*app.lambda, from.clone(), to.clone());
            let argm = substitution(*app.argm, from, to);

            Expr::App(App {
                lambda: Box::new(lambda),
                argm: Box::new(argm),
                ..app
            })
        }
        Expr::Abs(abs) => {
            let free = free_variables(to.clone());
            let cond = from != abs.param && !free.contains(&abs.param);

            if cond {
                let body = substitution(*abs.body, from, to);
                return Expr::Abs(Abs { body: Box::new(body), ..abs });
            }

            Expr::Abs(abs)
        }
        Expr::TAbs(TAbs { param, body, range }) => {
            let body = substitution(*body, from, to);
            Expr::TAbs(TAbs { param, body: Box::new(body), range })
        }
        Expr::TApp(TApp { lambda, argm, range }) => {
            let lambda = substitution(*lambda, from, to);
            Expr::TApp(TApp { lambda: Box::new(lambda), argm, range })
        }
    }
}

pub fn type_substitution(expr: &Expr, from: &str, to: &Type) -> Expr {
    match expr {
        Expr::App(app) => {
            let lambda = type_substitution(&app.lambda, from, to);
            let argm = type_substitution(&app.argm, from, to);

            Expr::App(App {
                lambda: Box::new(lambda),
                argm: Box::new(argm),
                ..app.clone()
            })
        }
        Expr::Abs(abs) => {
            let param_ty = type_type_substitute(&abs.param_ty, from, to);
            let body = type_substitution(&abs.body, from, to);

            return Expr::Abs(Abs {
                param_ty: param_ty,
                body: Box::new(body),
                ..abs.clone()
            });
        }
        Expr::TAbs(tabs) if tabs.param == from => expr.clone(),
        Expr::TAbs(TAbs { param, body, range }) => {
            let body = type_substitution(&body, from, to);

            Expr::TAbs(TAbs {
                param: param.clone(),
                body: Box::new(body),
                range: range.clone(),
            })
        }
        _ => expr.clone(),
    }
}

pub fn type_type_substitute(ty: &Type, from: &str, to: &Type) -> Type {
    match ty {
        Type::TVar { value } if value == from => to.clone(),
        Type::TVar { .. } => ty.clone(),
        Type::Arrow { left, right } => Type::Arrow {
            left: Box::new(type_type_substitute(left, from, to)),
            right: Box::new(type_type_substitute(right, from, to)),
        },
        Type::Forall { param, .. } if param == from => ty.clone(),
        Type::Forall { param, body } => Type::Forall {
            param: param.clone(),
            body: Box::new(type_type_substitute(body, from, to)),
        },
        _ => ty.clone(),
    }
}
