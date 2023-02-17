use super::variables::substitution;
use crate::parser::parsetree::{Abs, App, Expr};

macro_rules! break_limit {
    ($expr: ident, $limit: ident) => {
        if let Some(count) = $limit {
            if count == 0 {
                return $expr;
            }
        }
    };
}

pub fn normal_order(ex: Expr, limit: Option<usize>) -> Expr {
    break_limit!(ex, limit);

    match ex {
        Expr::Int(int) => Expr::Int(int),
        Expr::Var(var) => Expr::Var(var),
        Expr::Abs(abs) => {
            let body = normal_order(*abs.body, limit);

            Expr::Abs(Abs { body: Box::new(body), ..abs })
        }
        Expr::App(app) => {
            let func_expr = normal_order(*app.lambda, limit);

            match func_expr {
                Expr::Abs(abs) => {
                    let substituted = substitution(*abs.body, abs.param, *app.argm);
                    normal_order(substituted, limit.and_then(|l| Some(l - 1)))
                }
                expr => {
                    let func = normal_order(expr, limit);
                    let argm = normal_order(*app.argm, limit);

                    Expr::App(App {
                        lambda: Box::new(func),
                        argm: Box::new(argm),
                        ..app
                    })
                }
            }
        }
    }
}

pub fn applicative_order(ex: Expr, limit: Option<usize>) -> Expr {
    break_limit!(ex, limit);

    match ex {
        Expr::Int(int) => Expr::Int(int),
        Expr::Var(var) => Expr::Var(var),
        Expr::Abs(abs) => {
            let body = applicative_order(*abs.body, limit);
            Expr::Abs(Abs { body: Box::new(body), ..abs })
        }
        Expr::App(App { lambda, argm, range }) => {
            let func_expr = applicative_order(*lambda, limit);
            let argm_expr = applicative_order(*argm, limit);

            match func_expr {
                Expr::Abs(abs) => {
                    let substituted = substitution(*abs.body, abs.param, argm_expr);
                    applicative_order(substituted, limit.and_then(|l| Some(l - 1)))
                }
                expr => Expr::App(App {
                    lambda: Box::new(expr),
                    argm: Box::new(argm_expr),
                    range,
                }),
            }
        }
    }
}

pub fn call_by_name(ex: Expr, limit: Option<usize>) -> Expr {
    break_limit!(ex, limit);

    match ex {
        Expr::Int(int) => Expr::Int(int),
        Expr::Var(var) => Expr::Var(var),
        Expr::Abs(abs) => Expr::Abs(abs),
        Expr::App(App { lambda, argm, range }) => {
            let func_expr = call_by_name(*lambda, limit);

            match func_expr {
                Expr::Abs(abs) => {
                    let substituted = substitution(*abs.body, abs.param, *argm);
                    call_by_name(substituted, limit.and_then(|l| Some(l - 1)))
                }
                expr => Expr::App(App { lambda: Box::new(expr), argm, range }),
            }
        }
    }
}

pub fn call_by_value(ex: Expr, limit: Option<usize>) -> Expr {
    break_limit!(ex, limit);

    match ex {
        Expr::Int(int) => Expr::Int(int),
        Expr::Var(var) => Expr::Var(var),
        Expr::Abs(abs) => Expr::Abs(abs),
        Expr::App(App { lambda, argm, range }) => {
            let func_expr = call_by_value(*lambda, limit);

            match func_expr {
                Expr::Abs(abs) => {
                    let argm_expr = call_by_value(*argm, limit);
                    let substituted = substitution(*abs.body, abs.param, argm_expr);
                    call_by_name(substituted, limit.and_then(|l| Some(l - 1)))
                }
                expr => {
                    let argm_expr = call_by_value(*argm, limit);

                    Expr::App(App {
                        lambda: Box::new(expr),
                        argm: Box::new(argm_expr),
                        range,
                    })
                }
            }
        }
    }
}
