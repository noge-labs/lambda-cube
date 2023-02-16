use crate::parser::parsetree::{Abs, App, Expr, Var};
use std::collections::HashSet;

pub fn free_variables(expr: Expr) -> HashSet<String> {
    let mut free: HashSet<String> = HashSet::new();

    match expr {
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
    }

    free
}

pub fn substitution(expr: Expr, from: String, to: Expr) -> Expr {
    match expr.clone() {
        Expr::Var(Var { value, .. }) if value == from => to,
        Expr::Var(Var { .. }) => expr,
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
    }
}
