// use sistem_f::reduction;
use system_f::checker;
use system_f::parser;

fn main() {
    let input = r"(λX. λid: X. id) [Int]";
    let expr_parsed = parser::from_string(input).unwrap();
    let expr_typed = checker::type_of(expr_parsed).unwrap();

    println!("{}", expr_typed)

    // let expr_reduced = reduction::reduce(reduction::Norm::NOR, expr_parsed, None);

    // println!("{} : {}", expr_reduced, expr_typed)
}
