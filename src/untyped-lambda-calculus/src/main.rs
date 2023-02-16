use untyped_lambda_calculus::parser;
use untyped_lambda_calculus::reduction;

fn main() {
    let input = r"(λf. f f) (λx. x)";
    let parse = parser::from_string(input);

    match parse {
        Err(err) => panic!("{}", err),
        Ok(expr) => {
            let reduce = reduction::reduce(reduction::Norm::NOR, expr, None);
            println!("{}", reduce)
        }
    }
}
