use simply_typed_lambda_calculus::checker;
use simply_typed_lambda_calculus::parser;
use simply_typed_lambda_calculus::reduction;

fn main() {
    let input = r"(λf: int -> int. f) (λx: int. x)";

    let expr_parsed = parser::from_string(input).unwrap();
    let expr_typed = checker::type_of(&expr_parsed).unwrap();
    let expr_reduced = reduction::reduce(reduction::Norm::NOR, expr_parsed, None);

    println!("{} : {}", expr_reduced, expr_typed)
}
