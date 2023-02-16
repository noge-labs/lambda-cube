use crate::parser::parsetree::Type;

#[derive(Debug)]
pub enum TypeError {
    Mismatch(Type, Type),
    UndefinedVariable(String),
    UnexpectedType(Type),
}
