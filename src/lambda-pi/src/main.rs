// use lambda_pi::reduction;
// use lambda_pi::checker;
use lambda_pi::parser;

fn main() {
    let input = r"λa: A. a";

    let expr_parsed = parser::from_string(input).unwrap();
    // let expr_typed = checker::type_of(expr_parsed.clone()).unwrap();
    // let expr_reduced = reduction::reduce(reduction::Norm::NOR, expr_parsed, None);

    println!("{}", expr_parsed)
}
