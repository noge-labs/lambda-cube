use crate::checker::errors::TypeError;
use crate::parser::parsetree::{
    Abs, Anno, App, Arrow, Expr, Forall, Int, KindAlias, LetAlias, TAbs, TApp, TVar, TyAbs, TyAnno,
    TyApp, Type, TypeAlias, Var,
};
use crate::parser::symbol::Symbol;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Names {
    pub names: HashMap<String, Symbol>,
    pub count: usize,
}

impl Names {
    pub fn new() -> Names {
        Names { names: HashMap::new(), count: 0 }
    }

    pub fn rename(&mut self, name: &Symbol) -> Symbol {
        let new_count = self.count;
        let new_ident = Symbol { id: new_count, ..name.clone() };

        self.count += 1;
        self.names.insert(name.name.clone(), new_ident.clone());

        new_ident
    }
}

// I don't like de bruijn index.
pub fn alpha_conversion_type(context: &mut Names, ty: &Type) -> Result<Type, TypeError> {
    match ty {
        Type::TInt(_) => Ok(ty.clone()),
        Type::TVar(TVar { value }) => {
            if let Some(n) = context.names.get(&value.name) {
                Ok(Type::TVar(TVar { value: n.to_owned() }))
            } else {
                Err(TypeError::UndefinedVariable(value.name.clone()))
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
        Type::Forall(Forall { param, param_ty, body }) => {
            let param = context.rename(param);
            let body = alpha_conversion_type(context, body)?;

            Ok(Type::Forall(Forall {
                param,
                param_ty: param_ty.clone(),
                body: Box::new(body),
            }))
        }
        Type::TyAbs(TyAbs { param, param_ty, body }) => {
            let param = context.rename(param);
            let body = alpha_conversion_type(context, body)?;

            Ok(Type::TyAbs(TyAbs {
                param,
                param_ty: param_ty.clone(),
                body: Box::new(body),
            }))
        }
        Type::TyApp(TyApp { lambda, argm }) => {
            let lambda = alpha_conversion_type(context, lambda)?;
            let argm = alpha_conversion_type(context, argm)?;

            Ok(Type::TyApp(TyApp {
                lambda: Box::new(lambda),
                argm: Box::new(argm),
            }))
        }
        Type::TyAnno(TyAnno { ty, anno, .. }) => {
            let ty = alpha_conversion_type(context, ty)?;

            Ok(Type::TyAnno(TyAnno {
                ty: Box::new(ty),
                anno: anno.clone(),
            }))
        }
    }
}

pub fn alpha_conversion_expr(context: &mut Names, ex: &Expr) -> Result<Expr, TypeError> {
    match ex {
        Expr::Int(Int { .. }) => Ok(ex.clone()),
        Expr::Var(Var { value, range }) => {
            if let Some(n) = context.names.get(&value.name) {
                Ok(Expr::Var(Var { value: n.to_owned(), range: range.clone() }))
            } else {
                Err(TypeError::UndefinedVariable(value.name.clone()))
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
        Expr::TAbs(TAbs { param, param_ty, body, range }) => {
            let param = context.rename(param);
            let body = alpha_conversion_expr(context, &body)?;

            Ok(Expr::TAbs(TAbs {
                param,
                param_ty: param_ty.clone(),
                body: Box::new(body),
                range: range.clone(),
            }))
        }
        Expr::LetAlias(LetAlias { name, value, body, range }) => {
            let name = context.rename(name);
            let value = alpha_conversion_expr(context, value)?;
            let body = alpha_conversion_expr(context, body)?;

            Ok(Expr::LetAlias(LetAlias {
                name,
                value: Box::new(value),
                body: Box::new(body),
                range: range.clone(),
            }))
        }
        Expr::TypeAlias(TypeAlias { name, value, body, range }) => {
            let name = context.rename(name);
            let value = alpha_conversion_type(context, value)?;
            let body = alpha_conversion_expr(context, body)?;

            Ok(Expr::TypeAlias(TypeAlias {
                name,
                value: value,
                body: Box::new(body),
                range: range.clone(),
            }))
        }
        Expr::KindAlias(KindAlias { name, value, body, range }) => {
            let name = context.rename(name);
            let body = alpha_conversion_expr(context, body)?;

            Ok(Expr::KindAlias(KindAlias {
                name,
                value: value.clone(),
                body: Box::new(body),
                range: range.clone(),
            }))
        }
        Expr::Anno(Anno { expr, anno, range }) => {
            let expr = alpha_conversion_expr(context, expr)?;
            let anno = alpha_conversion_type(context, anno)?;

            Ok(Expr::Anno(Anno {
                expr: Box::new(expr),
                anno: anno.clone(),
                range: range.clone(),
            }))
        }
    }
}
