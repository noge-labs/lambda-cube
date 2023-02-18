pub mod error;

use std::sync::atomic::AtomicUsize;
use std::{collections::HashMap, sync::atomic::Ordering};

use crate::checker::error::TypeError;
use crate::parser::parsetree::{
    Abs, App, Arrow, Expr, Forall, Int, TAbs, TApp, TInt, TVar, Type, Var,
};

#[derive(Debug)]
pub struct Context {
    pub types: HashMap<String, Type>,
    pub names: HashMap<String, String>,
    pub count: AtomicUsize,
}

impl Context {
    pub fn new() -> Context {
        Context {
            types: HashMap::new(),
            names: HashMap::new(),
            count: AtomicUsize::new(0),
        }
    }

    pub fn rename(&mut self, name: &str) -> String {
        let new_id = self.count.fetch_add(1, Ordering::SeqCst);
        let new_name = format!("{}:{}", name, new_id);
        self.names.insert(name.to_string(), new_name.clone());

        new_name
    }
}

impl Default for Context {
    fn default() -> Context {
        Context::new()
    }
}

// I don't like de bruijn index.
pub fn alpha_conversion_type(context: &mut Context, ty: &Type) -> Result<Type, TypeError> {
    match ty {
        Type::TInt(_) => Ok(ty.clone()),
        Type::TVar(TVar { value }) => {
            if let Some(n) = context.names.get(value) {
                Ok(Type::TVar(TVar { value: n.to_owned() }))
            } else {
                Err(TypeError::UndefinedVariable(value.clone()))
            }
        }
        Type::Arrow(Arrow { left, right }) => {
            let left = alpha_conversion_type(context, left)?;
            let right = alpha_conversion_type(context, right)?;

            Ok(Type::Arrow(Arrow {
                left: Box::new(left),
                right: Box::new(right),
            }))
        }
        Type::Forall(Forall { param, body }) => {
            let param = context.rename(param);
            let body = alpha_conversion_type(context, body)?;

            Ok(Type::Forall(Forall { param, body: Box::new(body) }))
        }
    }
}

pub fn alpha_conversion_expr(context: &mut Context, ex: &Expr) -> Result<Expr, TypeError> {
    match ex {
        Expr::Int(Int { .. }) => Ok(ex.clone()),
        Expr::Var(Var { value, range }) => {
            if let Some(n) = context.names.get(value) {
                Ok(Expr::Var(Var { value: n.to_owned(), range: range.clone() }))
            } else {
                Err(TypeError::UndefinedVariable(value.clone()))
            }
        }
        Expr::App(App { lambda, argm, range }) => {
            let lambda = alpha_conversion_expr(context, lambda)?;
            let argm = alpha_conversion_expr(context, argm)?;

            Ok(Expr::App(App {
                lambda: Box::new(lambda),
                argm: Box::new(argm),
                range: range.clone(),
            }))
        }
        Expr::Abs(Abs { param, body, param_ty, range }) => {
            let param = context.rename(param);
            let param_ty = alpha_conversion_type(context, param_ty)?;
            let body = alpha_conversion_expr(context, &body)?;

            Ok(Expr::Abs(Abs {
                param,
                param_ty,
                body: Box::new(body),
                range: range.clone(),
            }))
        }
        Expr::TApp(TApp { lambda, argm, range }) => {
            let lambda = alpha_conversion_expr(context, lambda)?;
            let argm = alpha_conversion_type(context, argm)?;

            Ok(Expr::TApp(TApp {
                lambda: Box::new(lambda),
                argm,
                range: range.clone(),
            }))
        }
        Expr::TAbs(TAbs { param, body, range }) => {
            let param = context.rename(param);
            let body = alpha_conversion_expr(context, &body)?;

            Ok(Expr::TAbs(TAbs {
                param,
                body: Box::new(body),
                range: range.clone(),
            }))
        }
    }
}

pub fn equal(received: &Type, expected: &Type) -> bool {
    match (received, expected) {
        (Type::TVar(TVar { value: received, .. }), Type::TVar(TVar { value: expected, .. })) => {
            received == expected
        }
        (Type::TInt(TInt {}), Type::TInt(TInt {})) => true,
        (
            Type::Arrow(Arrow { left: received_left, right: received_right }),
            Type::Arrow(Arrow { left: expected_left, right: expected_right }),
        ) => equal(received_left, expected_left) & equal(received_right, expected_right),
        (
            Type::Forall(Forall { param: received_param, body: received_body }),
            Type::Forall(Forall { param: expected_param, body: expected_body }),
        ) => {
            let to = Type::TVar(TVar { value: expected_param.clone() });
            let received_bodya = substitution(*received_body.clone(), received_param.clone(), to);
            equal(&received_bodya, expected_body)
        }
        (_, _) => false,
    }
}

pub fn substitution(ty: Type, from: String, to: Type) -> Type {
    match ty {
        Type::TInt(TInt {}) => Type::TInt(TInt {}),
        Type::TVar(TVar { value }) if value == from => to,
        Type::TVar(TVar { value }) => Type::TVar(TVar { value }),
        Type::Arrow(Arrow { left, right }) => {
            let left = substitution(*left, from.clone(), to.clone());
            let right = substitution(*right, from, to);

            Type::Arrow(Arrow { left: Box::new(left), right: Box::new(right) })
        }
        Type::Forall(Forall { param, body }) => {
            if param == from {
                return Type::Forall(Forall { param, body });
            } else {
                let new_body = substitution(*body, from, to);

                Type::Forall(Forall { param, body: Box::new(new_body) })
            }
        }
    }
}

pub fn infer_type(context: &mut Context, ex: &Expr) -> Result<Type, TypeError> {
    match ex {
        Expr::Int(Int { .. }) => Ok(Type::TInt(TInt {})),
        Expr::Var(Var { value, .. }) => {
            if let Some(ty) = context.types.get(value) {
                Ok(ty.clone())
            } else {
                Err(TypeError::UndefinedVariable(value.clone()))
            }
        }
        Expr::Abs(Abs { param, param_ty, body, .. }) => {
            context.types.insert(param.clone(), param_ty.clone());

            let body_ty = infer_type(context, body)?;

            Ok(Type::Arrow(Arrow {
                left: Box::new(param_ty.clone()),
                right: Box::new(body_ty),
            }))
        }
        Expr::App(App { lambda, argm, .. }) => {
            let lambda_ty = infer_type(context, &lambda)?;
            let argm_ty = infer_type(context, argm)?;

            match lambda_ty {
                Type::Arrow(Arrow { left, right }) if equal(&left, &argm_ty) => Ok(*right),
                Type::Arrow(Arrow { left, .. }) => Err(TypeError::Mismatch(*left, argm_ty)),
                _ => Err(TypeError::Mismatch(lambda_ty.clone(), argm_ty)),
            }
        }
        Expr::TAbs(TAbs { param, body, .. }) => {
            let body_ty = infer_type(context, body)?;

            Ok(Type::Forall(Forall {
                param: param.clone(),
                body: Box::new(body_ty),
            }))
        }
        Expr::TApp(TApp { lambda, argm, .. }) => {
            let lambda_ty = infer_type(context, &lambda)?;

            if let Type::Forall(Forall { param, body }) = lambda_ty {
                Ok(substitution(*body, param.clone(), argm.clone()))
            } else {
                Err(TypeError::UnexpectedType(lambda_ty))
            }
        }
    }
}

pub fn type_of(ex: Expr) -> Result<Type, TypeError> {
    let mut context = Context::default();
    let alpha_terms = alpha_conversion_expr(&mut context, &ex)?;
    let typed_terms = infer_type(&mut context, &alpha_terms)?;

    Ok(typed_terms)
}
