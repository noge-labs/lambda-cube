use simply_typed_lambda_calculus::checker;
use simply_typed_lambda_calculus::parser;

fn main() {
    let input = r"(λf: int -> int. f)";
    let expr_parse = parser::from_string(input).unwrap();
    let expr_typed = checker::type_of(expr_parse);

    match expr_typed {
        Err(err) => panic!("{:?}", err),
        Ok(ty) => print!("{}", ty),
    }
}
