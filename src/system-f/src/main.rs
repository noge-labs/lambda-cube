use system_f::checker;
use system_f::parser;
use system_f::reduction;

fn main() {
    let input = r"
    let swap: ∀A. ∀B. ((A * B) -> (B * A)) = λA. λB. λpair: (A * B). {snd pair, fst pair} in
    swap [Int] [Int] {1, 2}
    ";

    let expr_parsed = parser::from_string(input).unwrap();
    let expr_typed = checker::type_of(expr_parsed.clone()).unwrap();
    let expr_reduced = reduction::reduce(reduction::Norm::NOR, expr_parsed, None);

    println!("({}) : {}", expr_reduced, expr_typed)
}
