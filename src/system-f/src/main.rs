use system_f::checker;
use system_f::parser;
use system_f::reduction;

fn main() {
    // let swap: ∀A. ∀B. ((A * B) -> (B * A)) = λA. λB. λp: (A * B). (snd p, fst p) in swap
    // let input = r"λA. λB. λx: A. λy: B. (x * y)";
    let input = "(λA. λB. λx: A. λy: B. x) [Bool]";

    let expr_parsed = parser::from_string(input).unwrap();
    let expr_typed = checker::type_of(expr_parsed.clone()).unwrap();
    let expr_reduced = reduction::reduce(reduction::Norm::NOR, expr_parsed, None);

    println!("({}) : {}", expr_reduced, expr_typed)
}
