use system_f::checker;
use system_f::parser;
use system_f::reduction;

fn main() {
    let input = r"let id: forall T. T -> T = (λT. λx: T. x) in id [Int] 69420";

    let expr_parsed = parser::from_string(input).unwrap();
    let expr_typed = checker::type_of(expr_parsed.clone()).unwrap();
    let expr_reduced = reduction::reduce(reduction::Norm::NOR, expr_parsed, None);

    println!("{} : {}", expr_reduced, expr_typed)
}
