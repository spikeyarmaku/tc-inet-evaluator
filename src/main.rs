mod agent;
mod code;
mod expr;
mod global;
mod parse;
mod rules;
mod vm;

use crate::vm::*;

fn assert_test(input: &str, expected_output: &str) -> (String, bool) {
    let tree = parse::parse_tree(input.to_string()).expect("expr_to_code: not a tree expression");
    let mut vm = VM::from_expr(tree);
    vm.eval();
    let output = vm.readback().to_string();
    (output.clone(), output == expected_output)
}

fn print_test(input: &str, expected_output: &str) {
    let (result, is_expected) = assert_test("tt(tt)t", "t");
    if is_expected {
        println!("Test: {} -> {}: {}", input, expected_output, is_expected);
    } else {
        println!("Test: {} -> {}: {} ({})", input, expected_output, is_expected, result);
    }
}

fn main() {
    print_test("tt(tt)t", "t");
    print_test("tt(tt(ttt))(tt)", "tt(ttt)");
}
