pub mod error;
pub mod expr;
pub mod lexer;
pub mod location;
pub mod macros;
pub mod parsetree;
pub mod state;

use lexer::state::*;

pub fn from_string(str: &str) -> Result<parsetree::Expr, error::ParserError> {
    let mut string = str.to_string();
    let mut parser = state::Parser::init(&mut string)?;

    return parser.parse_expr();
}
