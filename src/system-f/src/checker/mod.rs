pub mod error;

use std::collections::HashMap;

use crate::checker::error::TypeError;
use crate::parser::parsetree::{Abs, App, Expr, Int, TAbs, TApp, Type, Var};

#[derive(Debug)]
pub struct Context {
    pub types: HashMap<String, Type>,
    pub names: HashMap<String, String>,
    pub count: usize,
}

impl Context {
    pub fn new() -> Context {
        Context {
            types: HashMap::new(),
            names: HashMap::new(),
            count: 0,
        }
    }

    pub fn rename(&mut self, name: &str) -> String {
        let new_count = self.count;
        let new_ident = format!("{}{}", name, new_count);

        self.count += 1;
        self.names.insert(name.to_string(), new_ident.clone());

        new_ident
    }
}

impl Default for Context {
    fn default() -> Context {
        Context::new()
    }
}

pub fn alpha_conversion_type(context: &mut Context, ty: &Type) -> Result<Type, TypeError> {
    match ty {
        Type::TInt => Ok(ty.clone()),
        Type::TVar { value } => {
            if let Some(n) = context.names.get(value) {
                Ok(Type::TVar { value: n.clone() })
            } else {
                Err(TypeError::UndefinedVariable(value.clone()))
            }
        }
        Type::Arrow { left, right } => {
            let left = alpha_conversion_type(context, left)?;
            let right = alpha_conversion_type(context, right)?;

            Ok(Type::Arrow { left: Box::new(left), right: Box::new(right) })
        }
        Type::Forall { param, body } => {
            let param = context.rename(param);
            let body = alpha_conversion_type(context, body)?;

            Ok(Type::Forall { param, body: Box::new(body) })
        }
    }
}

pub fn alpha_conversion_expr(context: &mut Context, ex: &Expr) -> Result<Expr, TypeError> {
    match ex {
        Expr::Int(Int { .. }) => Ok(ex.clone()),
        Expr::Var(var) => {
            if let Some(n) = context.names.get(&var.value) {
                Ok(Expr::Var(Var { value: n.clone(), ..var.clone() }))
            } else {
                Err(TypeError::UndefinedVariable(var.value.clone()))
            }
        }
        Expr::App(app) => {
            let lambda = alpha_conversion_expr(context, &app.lambda)?;
            let argm = alpha_conversion_expr(context, &app.argm)?;

            Ok(Expr::App(App {
                lambda: Box::new(lambda),
                argm: Box::new(argm),
                ..app.clone()
            }))
        }
        Expr::Abs(abs) => {
            let param = context.rename(&abs.param);
            let param_ty = alpha_conversion_type(context, &abs.param_ty)?;
            let body = alpha_conversion_expr(context, &abs.body)?;

            Ok(Expr::Abs(Abs {
                param,
                param_ty,
                body: Box::new(body),
                ..abs.clone()
            }))
        }
        Expr::TApp(tapp) => {
            let lambda = alpha_conversion_expr(context, &tapp.lambda)?;
            let argm = alpha_conversion_type(context, &tapp.argm)?;

            Ok(Expr::TApp(TApp {
                lambda: Box::new(lambda),
                argm,
                ..tapp.clone()
            }))
        }
        Expr::TAbs(tabs) => {
            let param = context.rename(&tabs.param);
            let body = alpha_conversion_expr(context, &tabs.body)?;

            Ok(Expr::TAbs(TAbs {
                param,
                body: Box::new(body),
                ..tabs.clone()
            }))
        }
    }
}

pub fn equal(received: &Type, expected: &Type) -> bool {
    match (received, expected) {
        (Type::TInt, Type::TInt) => true,
        (Type::TVar { value: received, .. }, Type::TVar { value: expected, .. }) => {
            received == expected
        }
        (
            Type::Arrow { left: received_left, right: received_right },
            Type::Arrow { left: expected_left, right: expected_right },
        ) => equal(received_left, expected_left) & equal(received_right, expected_right),
        (
            Type::Forall { param: received_param, body: received_body },
            Type::Forall { param: expected_param, body: expected_body },
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
        Type::TVar { value } if value == from => to.clone(),
        Type::TVar { .. } => ty.clone(),
        Type::Arrow { left, right } => {
            let left = substitution(left, from, to);
            let right = substitution(right, from, to);

            Type::Arrow { left: Box::new(left), right: Box::new(right) }
        }
        Type::Forall { param, .. } if param == from => ty.clone(),
        Type::Forall { param, body } => {
            let body = substitution(body, from, to);

            Type::Forall { param: param.clone(), body: Box::new(body) }
        }
    }
}

pub fn infer_type(context: &mut Context, ex: &Expr) -> Result<Type, TypeError> {
    match ex {
        Expr::Int { .. } => Ok(Type::TInt),
        Expr::Var(var) => {
            if let Some(ty) = context.types.get(&var.value) {
                Ok(ty.clone())
            } else {
                Err(TypeError::UndefinedVariable(var.value.clone()))
            }
        }
        Expr::Abs(abs) => {
            context
                .types
                .insert(abs.param.clone(), abs.param_ty.clone());

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
                _ => Err(TypeError::Mismatch(lambda_ty.clone(), argm_ty)),
            }
        }
        Expr::TAbs(tabs) => {
            let body_ty = infer_type(context, &tabs.body)?;

            Ok(Type::Forall { param: tabs.param.clone(), body: Box::new(body_ty) })
        }
        Expr::TApp(tapp) => {
            let lambda_ty = infer_type(context, &tapp.lambda)?;

            if let Type::Forall { param, body } = lambda_ty {
                Ok(substitution(&body, &param, &tapp.argm))
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
