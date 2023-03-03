use system_f::checker;
use system_f::parser;
use system_f::reduction;

fn main() {
    let input = r"
    val swap : ∀A. ∀B. ((A * B) -> (B * A))
    let swap = λA. λB. λp: (A * B). (snd p, fst p) in
    
    val swap_int : ((Int * Int) -> (Int * Int))
    let swap_int = swap [Int] [Int] in
    
    val swap_bool : ((Bool * Bool) -> (Bool * Bool))
    let swap_bool = swap [Bool] [Bool] in
    
    swap_int true false
    ";

    let expr_parsed = parser::from_string(input).unwrap();
    let expr_typed = checker::type_of(expr_parsed.clone()).unwrap();
    let expr_reduced = reduction::reduce(reduction::Norm::NOR, expr_parsed, None);

    println!("({}) : {}", expr_reduced, expr_typed)
}
