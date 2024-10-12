mod agent;
mod code;
mod containers;
mod expr;
mod global;
mod parse;
mod rules;
mod vm;

use crate::vm::*;

fn eval(expr: &str) -> String {
    let tree = parse::parse_tree(expr).expect("expr_to_code: not a tree expression");
    let mut vm = VM::from_expr(tree);
    vm.eval();
    vm.readback().to_string()
}

fn main () {
    println!("{}", eval("t(tt)(tt)t")); // tt(ttt)
}