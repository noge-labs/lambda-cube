pub mod error;

use std::collections::HashMap;

use crate::checker::error::TypeError;
use crate::parser::parsetree::{Expr, Type};

pub struct Context {
    pub types: HashMap<String, Type>,
}

impl Context {
    pub fn new() -> Context {
        Context { types: HashMap::new() }
    }
}

// pub fn equal(received: &Type, expected: &Type) -> bool {
//     match (received, expected) {
//         (Type::TInt, Type::TInt) => true,
//         (Type::TBool, Type::TBool) => true,
//         (Type::TVar { value: received, .. }, Type::TVar { value: expected, .. }) => {
//             received == expected
//         }
//         (
//             Type::Arrow {
//                 left: received_left,
//                 right: received_right,
//             },
//             Type::Arrow {
//                 left: expected_left,
//                 right: expected_right,
//             },
//         ) => equal(received_left, expected_left) & equal(received_right, expected_right),
//         (
//             Type::Product {
//                 fst: received_fst,
//                 snd: received_snd,
//             },
//             Type::Product {
//                 fst: expected_fst,
//                 snd: expected_snd,
//             },
//         ) => equal(&received_fst, &expected_fst) & equal(&received_snd, &expected_snd),
//         (
//             Type::Forall {
//                 param: received_param,
//                 body: received_body,
//             },
//             Type::Forall {
//                 param: expected_param,
//                 body: expected_body,
//             },
//         ) => {
//             let to = Type::TVar { value: expected_param.clone() };
//             let received_body = substitution(received_body, received_param, &to);
//             equal(&received_body, expected_body)
//         }
//         (_, _) => false,
//     }
// }

pub fn equal(received: &Type, expected: &Type) -> bool {
    match (received, expected) {
        (Type::TInt, Type::TInt) => true,
        (Type::TBool, Type::TBool) => true,
        (Type::TVar { value: received, .. }, Type::TVar { value: expected, .. }) => {
            received == expected
        }
        (
            Type::Arrow {
                left: received_left,
                right: received_right,
            },
            Type::Arrow {
                left: expected_left,
                right: expected_right,
            },
        ) => equal(received_left, expected_left) & equal(received_right, expected_right),
        (
            Type::Product {
                fst: received_fst,
                snd: received_snd,
            },
            Type::Product {
                fst: expected_fst,
                snd: expected_snd,
            },
        ) => equal(&received_fst, &expected_fst) & equal(&received_snd, &expected_snd),
        (
            Type::Forall {
                param: received_param,
                body: received_body,
            },
            Type::Forall {
                param: expected_param,
                body: expected_body,
            },
        ) => {
            let to = Type::TVar { value: expected_param.clone() };
            let received_body = substitution(received_body, received_param, &to);
            equal(&received_body, expected_body)
        }
        (_, _) => false,
    }
}

pub fn substitution(ty: &Type, from: &str, to: &Type) -> Type {
    match ty {
        Type::TInt => ty.clone(),
        Type::TBool => ty.clone(),
        Type::TVar { value } if value == from => to.clone(),
        Type::TVar { .. } => ty.clone(),
        Type::Arrow { left, right } => {
            let left = substitution(left, from, to);
            let right = substitution(right, from, to);

            Type::Arrow {
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        Type::Forall { param, .. } if param == from => ty.clone(),
        Type::Forall { param, body } => {
            let body = substitution(body, from, to);

            Type::Forall {
                param: param.clone(),
                body: Box::new(body),
            }
        }
        Type::Product { fst, snd } => {
            let fst = substitution(fst, from, to);
            let snd = substitution(snd, from, to);

            Type::Product {
                fst: Box::new(fst),
                snd: Box::new(snd),
            }
        }
    }
}

pub fn infer_type(context: &mut Context, ex: &Expr) -> Result<Type, TypeError> {
    match ex {
        Expr::Int { .. } => Ok(Type::TInt),
        Expr::Bool { .. } => Ok(Type::TBool),
        Expr::Var(var) => match context.types.get(&var.value) {
            Some(ty) => Ok(ty.clone()),
            None => Err(TypeError::UndefinedVariable(var.value.clone())),
        },
        Expr::Pair(pair) => {
            let fst = infer_type(context, &pair.fst)?;
            let snd = infer_type(context, &pair.snd)?;

            Ok(Type::Product {
                fst: Box::new(fst),
                snd: Box::new(snd),
            })
        }
        Expr::Fst(fst) => {
            let pair = infer_type(context, &fst.pair)?;

            match pair {
                Type::Product { fst, .. } => Ok(*fst),
                pair => Err(TypeError::UnexpectedType(pair)),
            }
        }
        Expr::Snd(snd) => {
            let pair = infer_type(context, &snd.pair)?;

            match pair {
                Type::Product { snd, .. } => Ok(*snd),
                pair => Err(TypeError::UnexpectedType(pair)),
            }
        }
        Expr::Abs(abs) => {
            context.types.insert(abs.param.clone(), abs.param_ty.clone());

            let body_ty = infer_type(context, &abs.body)?;

            Ok(Type::Arrow {
                left: Box::new(abs.param_ty.clone()),
                right: Box::new(body_ty),
            })
        }
        Expr::App(app) => {
            let lambda_ty = infer_type(context, &app.lambda)?;
            let argm_ty = infer_type(context, &app.argm)?;

            match lambda_ty {
                Type::Arrow { left, right } if equal(&left, &argm_ty) => Ok(*right),
                Type::Arrow { left, .. } => Err(TypeError::Mismatch(*left, argm_ty)),
                _ => Err(TypeError::Mismatch(lambda_ty, argm_ty)),
            }
        }
        Expr::TAbs(tabs) => {
            let body_ty = infer_type(context, &tabs.body)?;

            Ok(Type::Forall {
                param: tabs.param.clone(),
                body: Box::new(body_ty),
            })
        }
        Expr::TApp(tapp) => {
            let lambda_ty = infer_type(context, &tapp.lambda)?;

            match lambda_ty {
                Type::Forall { param, body } => Ok(substitution(&body, &param, &tapp.argm)),
                _ => Err(TypeError::UnexpectedType(lambda_ty)),
            }
        }
    }
}

pub fn type_of(ex: Expr) -> Result<Type, TypeError> {
    let mut context = Context::new();
    let typed_terms = infer_type(&mut context, &ex)?;

    Ok(typed_terms)
}
