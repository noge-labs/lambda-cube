pub mod error;

use std::collections::HashMap;

use self::error::TypeError;
use crate::parser::parsetree::{Abs, App, Arrow, Expr, Int, TInt, Type, Var};

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

type TypeContext = HashMap<String, Type>;

fn infer_type(context: &mut TypeContext, expr: &Expr) -> Result<Type, TypeError> {
    match expr {
        Expr::Int(Int { .. }) => Ok(Type::TInt(TInt {})),
        Expr::Var(Var { value, .. }) => match context.get(value) {
            None => Err(TypeError::UndefinedVariable(value.clone())),
            Some(ty) => Ok(ty.clone()),
        },
        Expr::Abs(Abs { param, param_ty, body, .. }) => {
            context.insert(param.clone(), param_ty.clone());

            let body_ty = infer_type(context, body)?;

            Ok(Type::Arrow(Arrow {
                left: Box::new(param_ty.clone()),
                right: Box::new(body_ty),
            }))
        }
        Expr::App(App { lambda, argm, .. }) => {
            let lambda_ty = infer_type(context, lambda)?;
            let argm_ty = infer_type(context, argm)?;

            if let Type::Arrow(Arrow { left, right }) = lambda_ty {
                if equal_type(&left, &argm_ty) {
                    Ok(*right)
                } else {
                    Err(TypeError::Mismatch(*left, argm_ty))
                }
            } else {
                Err(TypeError::UnexpectedType(lambda_ty.clone()))
            }
        }
    }
}

pub fn type_of(expr: &Expr) -> Result<Type, TypeError> {
    infer_type(&mut HashMap::new(), expr)
}
