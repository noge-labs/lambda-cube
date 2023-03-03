use lambda_pi::parser;

fn main() {
    let input = r"λA: *. λB: Πa: A. *. λa: A. B a";

    let expr_parsed = parser::from_string(input).unwrap();

    println!("{}", expr_parsed)
}
