use crate::parser::parsetree::Expr;

pub mod strategies;
pub mod variables;

pub enum Norm {
    NOR, // normal-order
}

pub fn reduce(strategy: Norm, expr: Expr, limit: Option<usize>) -> Expr {
    let limit = Some(limit.unwrap_or(100));

    match strategy {
        Norm::NOR => strategies::normal_order(expr, limit),
    }
}
