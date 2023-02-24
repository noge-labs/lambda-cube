use system_f_omega::checker;
use system_f_omega::checker::conversion::{alpha_conversion_expr, Names};
use system_f_omega::parser;
// use system_f_omega::reduction;

fn main() {
    let input = r"
        type Id: * = ∀A: *. (A -> A) in
        let id: Id = λA: *. λx: A. x in
        id [Int] 69420
    ";

    let expr_parsed = parser::from_string(input).unwrap();
    let mut context = Names::new();
    let alpha_terms = alpha_conversion_expr(&mut context, &expr_parsed).unwrap();

    let expr_typed = checker::type_of(alpha_terms.clone()).unwrap();

    println!("{}", expr_typed)
}
