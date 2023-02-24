use super::errors::TypeError;
use super::substitution;
use super::typedtree as T;

pub fn check_type_equiv(received: &T::Annoted, expected: &T::Annoted) -> Result<(), TypeError> {
    match (*received.clone().desc, *expected.clone().desc) {
        (T::Type::Int, T::Type::Int) => Ok(()),
        (T::Type::Var { value: re_value, .. }, T::Type::Var { value: ex_value, .. })
            if re_value == ex_value =>
        {
            Ok(())
        }
        (T::Type::Var { value: _ }, T::Type::Var { value: _ }) => Err(TypeError::VariableClash),
        (
            T::Type::Arrow { left: re_left, right: re_right },
            T::Type::Arrow { left: ex_left, right: ex_right },
        ) => {
            check_type_equiv(&re_left, &ex_left)?;
            check_type_equiv(&re_right, &ex_right)
        }
        (
            T::Type::Forall { param: re_param, param_ty: re_kind, body: re_body },
            T::Type::Forall { param: ex_param, param_ty: ex_kind, body: ex_body },
        ) => {
            let to = T::Type::Var { value: ex_param };
            let substituted = substitution(re_body, re_param, to);

            check_kind_equiv(&re_kind, &ex_kind)?;
            check_type_equiv(&substituted, &ex_body)
        }
        (_, _) => Err(TypeError::TypeClash),
    }
}

pub fn check_kind_equiv(received: &T::Kind, expected: &T::Kind) -> Result<(), TypeError> {
    match (received, expected) {
        (T::Kind::Star, T::Kind::Star) => Ok(()),
        (
            T::Kind::KindArrow { left: received_left, right: received_right },
            T::Kind::KindArrow { left: expected_left, right: expected_right },
        ) => {
            check_kind_equiv(received_left, expected_left)?;
            check_kind_equiv(received_right, expected_right)
        }
        (_, _) => Err(TypeError::TypeClash),
    }
}
