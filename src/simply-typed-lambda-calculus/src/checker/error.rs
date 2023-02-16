use crate::parser::parsetree::Type;

#[derive(Debug)]
pub enum TypeError {
    TypeClash(Type, Type),
    UndefinedVariable(String),
    UnexpectedType(Type),
}
