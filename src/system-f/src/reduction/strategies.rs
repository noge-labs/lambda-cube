use super::variables::{substitution, type_substitution};
use crate::parser::parsetree::{Abs, App, Expr, Fst, Pair, Snd, TAbs, TApp};

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
        Expr::Pair(pair) => {
            let fst = normal_order(*pair.fst, limit);
            let snd = normal_order(*pair.snd, limit);

            Expr::Pair(Pair {
                fst: Box::new(fst),
                snd: Box::new(snd),
                ..pair
            })
        }
        Expr::Fst(fst) => {
            let pair = normal_order(*fst.pair, limit);

            match pair {
                Expr::Pair(Pair { fst, .. }) => *fst,
                pair => Expr::Fst(Fst { pair: Box::new(pair), ..fst }),
            }
        }
        Expr::Snd(snd) => {
            let pair = normal_order(*snd.pair, limit);

            match pair {
                Expr::Pair(Pair { snd, .. }) => *snd,
                pair => Expr::Snd(Snd { pair: Box::new(pair), ..snd }),
            }
        }
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
        Expr::TAbs(tabs) => {
            let body = normal_order(*tabs.body, limit);

            Expr::TAbs(TAbs {
                param: tabs.param,
                body: Box::new(body),
                range: tabs.range,
            })
        }
        Expr::TApp(tapp) => {
            let func_expr = normal_order(*tapp.lambda, limit);

            match func_expr {
                Expr::TAbs(TAbs { param, body, .. }) => {
                    let substituted = type_substitution(&body, &param, &tapp.argm);
                    normal_order(substituted, limit.and_then(|l| Some(l - 1)))
                }
                expr => {
                    let func = normal_order(expr, limit);

                    Expr::TApp(TApp {
                        lambda: Box::new(func),
                        argm: tapp.argm,
                        range: tapp.range,
                    })
                }
            }
        }
    }
}
