use super::context::Context;
use super::substitution;
use super::typedtree as T;

pub fn normalize(context: &mut Context, ty: T::Annoted) -> T::Annoted {
    let desc = match *ty.desc {
        T::Type::Int => T::Type::Int,
        T::Type::Var { value } => T::Type::Var { value },
        T::Type::Forall { param, param_ty, body } => {
            let body = normalize(context, body);

            T::Type::Forall { param, param_ty, body }
        }
        T::Type::Arrow { left, right } => {
            let left = normalize(context, left);
            let right = normalize(context, right);

            T::Type::Arrow { left, right }
        }
        T::Type::TyAbs { param, param_ty, body } => {
            let body = normalize(context, body);

            T::Type::TyAbs { param, param_ty, body }
        }
        T::Type::TyApp { lambda, argm } => {
            let lambda = normalize(context, lambda);
            let argm = normalize(context, argm.clone());
            let desc = normalize(context, argm.clone());

            match *desc.desc {
                T::Type::TyAbs { param, param_ty: _, body } => {
                    let argm = argm.desc;
                    let sub = substitution(body.clone(), param, *argm);
                    let body = normalize(context, T::Annoted { desc: sub.desc, kind: sub.kind });
                    *body.desc
                }
                _ => T::Type::TyApp { lambda, argm },
            }
        }
    };

    T::Annoted { desc: Box::new(desc), kind: ty.kind }
}
