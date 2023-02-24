use super::variables::{substitution, type_substitution};
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
