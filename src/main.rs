mod agent;
mod code;
mod containers;
mod expr;
mod global;
mod parse;
mod rules;
mod vm;

use crate::vm::*;

fn assert_test(input: &String, expected_output: &String) -> (String, bool) {
    let tree = parse::parse_tree(input).expect("expr_to_code: not a tree expression");
    let mut vm = VM::from_expr(tree);
    vm.eval();
    let output = vm.readback().to_string();
    (output.clone(), output == *expected_output)
}

fn print_test(input: String, expected_output: String) {
    let (result, is_expected) = assert_test(&input, &expected_output);
    if is_expected {
        println!("Test: {} | {} -> {}", is_expected, &input, expected_output);
    } else {
        println!(
            "Test: {} | expected: {} -> {}, got {}",
            is_expected, &input, &expected_output, result
        );
    }
}

fn test() {
    // Test the rules

    let a = "tt";
    let b = "t(tt)";
    let c = "t(t(tt))";
    let u = "ttt";
    let v = "t(tt)(tt)";
    // t t       a b       -> a
    print_test(format!("tt({a})({b})"), "tt".to_string());
    // t (t a)   b c       -> a c (b c)
    print_test(format!("t(t({a}))({b})({c})"), "t(t(tt))".to_string());
    // t (t a b) c t       -> a
    print_test(format!("t(t({a})({b}))({c})t"), "tt".to_string());
    // t (t a b) c (t u)   -> b u
    print_test(
        format!("t(t({a})({b}))({c})(t({u}))"),
        "t(tt)(ttt)".to_string(),
    );
    // t (t a b) c (t u v) -> c u v
    let c = "t";
    print_test(
        format!("t(t({a})({b}))({c})(t({u})({v}))"),
        "t(ttt)(t(tt)(tt))".to_string(),
    );
}

fn main () {
    // print_test("t(tttt)".to_string(), "tt".to_string());
    test();

    // print_test("tttt".to_string(), "t".to_string());
}