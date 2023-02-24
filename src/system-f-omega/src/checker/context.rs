use crate::checker::errors::TypeError;
use crate::parser::parsetree::{Expr, Kind, Type};
use crate::parser::symbol::Symbol;

use std::collections::HashMap;

use super::typedtree as T;

#[derive(Debug, Clone)]
pub enum ContextExpr {
    Value(T::Annoted),
    Alias(Expr),
}

#[derive(Debug, Clone)]
pub enum ContextType {
    Value(T::Kind),
    Alias(Type),
}

#[derive(Debug, Clone)]
pub struct Context {
    exprs: HashMap<Symbol, ContextExpr>,
    types: HashMap<Symbol, ContextType>,
    kinds: HashMap<Symbol, Kind>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            exprs: HashMap::new(),
            types: HashMap::new(),
            kinds: HashMap::new(),
        }
    }

    pub fn get_expr(&self, key: &Symbol) -> Result<ContextExpr, TypeError> {
        match self.exprs.get(key) {
            Some(expr) => Ok(expr.clone()),
            None => Err(TypeError::UndefinedVariable(key.to_string())),
        }
    }

    pub fn add_expr(&mut self, key: &Symbol, expr: T::Annoted) {
        self.exprs.insert(key.clone(), ContextExpr::Value(expr));
    }

    pub fn add_expr_alias(&mut self, key: &Symbol, expr: Expr) {
        self.exprs.insert(key.clone(), ContextExpr::Alias(expr));
    }

    pub fn add_type(&mut self, key: &Symbol, expr: T::Kind) {
        self.types.insert(key.clone(), ContextType::Value(expr));
    }

    pub fn add_type_alias(&mut self, key: &Symbol, expr: Type) {
        self.types.insert(key.clone(), ContextType::Alias(expr));
    }

    pub fn add_kind_alias(&mut self, key: &Symbol, expr: Kind) {
        self.kinds.insert(key.clone(), expr);
    }

    pub fn get_type(&self, key: &Symbol) -> Result<ContextType, TypeError> {
        match self.types.get(key) {
            Some(expr) => Ok(expr.clone()),
            None => Err(TypeError::UndefinedVariable(key.to_string())),
        }
    }

    pub fn get_kind(&self, key: &Symbol) -> Result<Kind, TypeError> {
        match self.kinds.get(key) {
            Some(expr) => Ok(expr.clone()),
            None => Err(TypeError::UndefinedVariable(key.to_string())),
        }
    }
}

impl Default for Context {
    fn default() -> Context {
        Context::new()
    }
}
