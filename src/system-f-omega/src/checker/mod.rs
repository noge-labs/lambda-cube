pub mod context;
pub mod conversion;
pub mod equivalence;
pub mod errors;
pub mod normalize;
pub mod typedtree;

use self::context::{Context, ContextExpr, ContextType};
use self::equivalence::{check_kind_equiv, check_type_equiv};
use self::errors::TypeError;
use self::normalize::normalize;
use self::typedtree as T;

use crate::parser::parsetree::{
    Abs, Anno, App, Arrow, Expr, Forall, Int, Kind, KindAlias, KindArrow, KindVar, LetAlias, Star,
    TAbs, TApp, TInt, TVar, TyAbs, TyAnno, TyApp, Type, TypeAlias, Var,
};
use crate::parser::symbol::Symbol;

pub fn transl_kind(context: &mut Context, kind: &Kind) -> T::Kind {
    match kind {
        Kind::Star(Star {}) => T::Kind::Star,
        Kind::KindVar(KindVar { value }) => match context.get_kind(&value) {
            Ok(kind) => transl_kind(context, &kind),
            Err(err) => panic!("{}", err),
        },
        Kind::KindArrow(KindArrow { left, right }) => {
            let left = transl_kind(context, left);
            let right = transl_kind(context, right);
            T::Kind::KindArrow { left: Box::new(left), right: Box::new(right) }
        }
    }
}

pub fn substitution(ty: T::Annoted, from: Symbol, to: T::Type) -> T::Annoted {
    let desc = match *ty.desc {
        T::Type::Int => T::Type::Int,
        T::Type::Var { value } if value == from => to,
        T::Type::Var { value } => T::Type::Var { value },
        T::Type::Arrow { left, right } => {
            let left = substitution(left, from.clone(), to.clone());
            let right = substitution(right, from, to);

            T::Type::Arrow { left, right }
        }
        T::Type::Forall { param, param_ty, body } if param == from => {
            T::Type::Forall { param, param_ty, body }
        }
        T::Type::Forall { param, param_ty, body } => {
            let body = substitution(body, from, to);
            T::Type::Forall { param, param_ty, body }
        }
        T::Type::TyAbs { param, param_ty, body } if param == from => {
            T::Type::Forall { param, param_ty, body }
        }
        T::Type::TyAbs { param, param_ty, body } => {
            let body = substitution(body, from, to);
            T::Type::Forall { param, param_ty, body }
        }
        T::Type::TyApp { lambda, argm } => {
            let lambda = substitution(lambda, from.clone(), to.clone());
            let argm = substitution(argm, from, to);

            T::Type::TyApp { lambda, argm }
        }
    };

    T::Annoted { desc: Box::new(desc), kind: ty.kind }
}

pub fn infer_type(context: &mut Context, ty: Type) -> Result<T::Annoted, TypeError> {
    match ty {
        Type::TInt(TInt {}) => Ok(T::Annoted { desc: Box::new(T::Type::Int), kind: T::Kind::Star }),
        Type::TyAnno(TyAnno { ty, anno, .. }) => {
            let annotation = transl_kind(context, &anno.clone());
            check_type(context, *ty, annotation)
        }
        Type::TVar(TVar { value }) => {
            let expr = context.get_type(&value);

            match expr {
                Err(error) => Err(error),
                Ok(ContextType::Alias(alias)) => infer_type(context, alias),
                Ok(ContextType::Value(kind)) => {
                    Ok(T::Annoted { desc: Box::new(T::Type::Var { value }), kind: kind })
                }
            }
        }
        Type::Forall(Forall { param, param_ty, body }) => {
            let param_ty = transl_kind(context, &param_ty);
            context.add_type(&param, param_ty.clone());
            let body = check_type(context, *body, T::Kind::Star)?;

            Ok(T::Annoted {
                desc: Box::new(T::Type::Forall { param, param_ty, body }),
                kind: T::Kind::Star,
            })
        }
        Type::Arrow(Arrow { left, right }) => {
            let left = check_type(context, *left, T::Kind::Star)?;
            let right = check_type(context, *right, T::Kind::Star)?;

            Ok(T::Annoted {
                desc: Box::new(T::Type::Arrow { left, right }),
                kind: T::Kind::Star,
            })
        }
        Type::TyAbs(TyAbs { param, param_ty, body }) => {
            let param_ty = transl_kind(context, &param_ty);
            context.add_type(&param, param_ty.clone());
            let body = check_type(context, *body, T::Kind::Star)?;

            Ok(T::Annoted {
                desc: Box::new(T::Type::TyAbs {
                    param,
                    param_ty: param_ty.clone(),
                    body: body.clone(),
                }),
                kind: T::Kind::KindArrow {
                    left: Box::new(param_ty),
                    right: Box::new(body.kind),
                },
            })
        }
        Type::TyApp(TyApp { lambda, argm }) => {
            let lambda = infer_type(context, *lambda)?;

            match lambda.clone().kind {
                T::Kind::Star => Err(TypeError::TypeClash),
                T::Kind::KindArrow { left, right } => {
                    let argm = check_type(context, *argm, *left)?;

                    Ok(T::Annoted {
                        desc: Box::new(T::Type::TyApp { lambda, argm }),
                        kind: *right,
                    })
                }
            }
        }
    }
}

pub fn infer_expr(context: &mut Context, ex: &Expr) -> Result<T::Annoted, TypeError> {
    match ex.clone() {
        Expr::Anno(Anno { expr, anno, .. }) => {
            let annotation = check_type(context, anno.clone(), T::Kind::Star)?;

            check_expr(context, *expr.clone(), annotation.clone())?;
            Ok(annotation)
        }
        Expr::LetAlias(LetAlias { name, value, body, .. }) => {
            context.add_expr_alias(&name, *value);
            infer_expr(context, &body.clone())
        }
        Expr::TypeAlias(TypeAlias { name, value, body, .. }) => {
            context.add_type_alias(&name, value.clone());
            infer_expr(context, &body.clone())
        }
        Expr::KindAlias(KindAlias { name, value, body, .. }) => {
            context.add_kind_alias(&name, value);
            infer_expr(context, &body)
        }
        Expr::Int(Int { .. }) => {
            Ok(T::Annoted { desc: Box::new(T::Type::Int), kind: T::Kind::Star })
        }
        Expr::Var(Var { value, .. }) => {
            let expr = context.get_expr(&value);

            match expr {
                Ok(ContextExpr::Value(value)) => Ok(value),
                Ok(ContextExpr::Alias(alias)) => infer_expr(context, &alias),
                Err(error) => Err(error),
            }
        }
        Expr::Abs(Abs { param, param_ty, body, .. }) => {
            let param_ty = check_type(context, param_ty, T::Kind::Star)?;
            context.add_expr(&param, param_ty.clone());
            let body_ty = infer_expr(context, &body.clone())?;

            Ok(T::Annoted {
                desc: Box::new(T::Type::Arrow { left: param_ty, right: body_ty }),
                kind: T::Kind::Star,
            })
        }
        Expr::App(App { lambda, argm, .. }) => {
            let lambda_ty = infer_expr(context, &lambda)?;
            let forall_ty = normalize(context, lambda_ty);

            match *forall_ty.desc {
                T::Type::Arrow { left, right } => {
                    check_expr(context, *argm, left)?;
                    Ok(right)
                }
                _ => Err(TypeError::TypeNotAArrow(*forall_ty.desc)),
            }
        }
        Expr::TAbs(TAbs { param, param_ty, body, .. }) => {
            let kind = transl_kind(context, &param_ty);
            context.add_type(&param, kind.clone());
            let body = infer_expr(context, &body)?;

            Ok(T::Annoted {
                desc: Box::new(T::Type::Forall { param, param_ty: kind, body }),
                kind: T::Kind::Star,
            })
        }
        Expr::TApp(TApp { lambda, argm, .. }) => {
            let lambda_ty = infer_expr(context, &lambda)?;
            let forall_ty = normalize(context, lambda_ty);

            match *forall_ty.desc {
                T::Type::Forall { param, param_ty, body } => {
                    let argm = check_type(context, argm, param_ty)?;
                    Ok(substitution(body, param.clone(), *argm.desc))
                }
                _ => Err(TypeError::TypeNotAForall(*forall_ty.desc)),
            }
        }
    }
}

pub fn check_type(
    context: &mut Context,
    ty: Type,
    expected: T::Kind,
) -> Result<T::Annoted, TypeError> {
    let received = infer_type(context, ty)?;
    check_kind_equiv(&received.kind, &expected)?;

    Ok(received)
}

pub fn check_expr(context: &mut Context, ex: Expr, expected: T::Annoted) -> Result<(), TypeError> {
    let forall_ty = normalize(context, expected.clone());

    match (ex.clone(), *forall_ty.desc) {
        (
            Expr::TAbs(TAbs { param: rp, param_ty: rt, body: rb, .. }),
            T::Type::Forall { param: ep, param_ty: ek, body: bk },
        ) => {
            let rt = transl_kind(context, &rt);
            check_kind_equiv(&rt, &ek)?;
            let ret = substitution(bk, ep, T::Type::Var { value: rp.clone() });
            context.add_type(&rp, ek);
            check_expr(context, *rb, ret)
        }
        (Expr::Abs(Abs { param, param_ty, body, .. }), T::Type::Arrow { left, right }) => {
            let received_param = check_type(context, param_ty, T::Kind::Star)?;
            check_type_equiv(&received_param, &left)?;
            context.add_expr(&param, left);

            check_expr(context, *body, right)
        }
        (expr, _) => {
            let received = infer_expr(context, &expr)?;
            check_type_equiv(&received, &expected)
        }
    }
}

pub fn type_of(ex: Expr) -> Result<T::Annoted, TypeError> {
    let mut context = Context::default();
    let typed_terms = infer_expr(&mut context, &ex)?;
    let norml_terms = normalize(&mut context, typed_terms);

    Ok(norml_terms)
}
