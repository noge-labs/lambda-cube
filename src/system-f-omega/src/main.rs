// use system_f_omega::checker;
use system_f_omega::parser;
// use system_f_omega::reduction;

fn main() {
    // type True : * -> * -> * = λt: *. λf: *. t in
    // type False : * -> * -> * = λt: *. λf: *. f in
    // type If: λpred: (* -> * -> *). λt: *. λf: * = pred t f in

    let input = r"
        kind Bool = * -> * -> * in
        type True: Bool = (λT: *. (λF: *. T)) in
        type False: Bool = (λT: *. (λF: *. F)) in
        let true: True = λt: (λA: *. A). λf: (λA: *. A). t in
        true
    ";

    let expr_parsed = parser::from_string(input).unwrap();
    println!("{}", expr_parsed)
    // let expr_typed = checker::type_of(expr_parsed.clone()).unwrap();
    // let expr_reduced = reduction::reduce(reduction::Norm::NOR, expr_parsed, None);

    // println!("{} : {}", expr_reduced, expr_typed)
}
