use crate::parser::parsetree::Expr;

pub mod strategies;
pub mod variables;

pub enum Norm {
    NOR, // normal-order
    APP, // applicative-order
    CBN, // call-by-name
    CBV, // call-by-value
}

pub fn reduce(strategy: Norm, expr: Expr, limit: Option<usize>) -> Expr {
    let limit = Some(limit.unwrap_or(100));

    match strategy {
        Norm::NOR => strategies::normal_order(expr, limit),
        Norm::APP => strategies::applicative_order(expr, limit),
        Norm::CBN => strategies::call_by_name(expr, limit),
        Norm::CBV => strategies::call_by_value(expr, limit),
    }
}
