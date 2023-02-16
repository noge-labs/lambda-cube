pub mod error;

use std::collections::HashMap;

use self::error::TypeError;
use crate::parser::parsetree::{Abs, App, Arrow, Expr, Type, Var};

fn equal_type(ty1: &Type, ty2: &Type) -> bool {
    match (ty1, ty2) {
        (Type::Arrow(Arrow { left, right }), Type::Arrow(Arrow { left: l0, right: r0 })) => {
            equal_type(left, l0) && equal_type(right, r0)
        }
        (Type::TInt(_), Type::TInt(_)) => true,
        (Type::Arrow(_), Type::TInt(_)) => false,
        (Type::TInt(_), Type::Arrow(_)) => false,
    }
}

fn infer_type(context: &mut HashMap<String, Type>, expr: Expr) -> Result<Type, TypeError> {
    match expr {
        Expr::Var(Var { value, .. }) => match context.get(&value) {
            Some(ty) => Ok(ty.clone()),
            None => Err(TypeError::UndefinedVariable(value)),
        },
        Expr::Abs(Abs { param, param_ty, body, .. }) => {
            context.entry(param.clone()).or_insert(param_ty.clone());

            let body_ty = infer_type(context, *body)?;

            Ok(Type::Arrow(Arrow {
                left: Box::new(param_ty),
                right: Box::new(body_ty),
            }))
        }
        Expr::App(App { lambda, argm, .. }) => {
            let abs_ty = infer_type(context, *lambda)?;
            let arg_ty = infer_type(context, *argm)?;

            if let Type::Arrow(Arrow { left, right }) = abs_ty {
                if equal_type(&left, &arg_ty) {
                    Ok(*right)
                } else {
                    Err(TypeError::TypeClash(*left, arg_ty))
                }
            } else {
                Err(TypeError::UnexpectedType(abs_ty.clone()))
            }
        }
    }
}

pub fn type_of(expr: Expr) -> Result<Type, TypeError> {
    infer_type(&mut HashMap::new(), expr)
}
