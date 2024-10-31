mod agent;
mod code;
mod compiler;
mod containers;
mod expr;
mod global;
mod parse;
mod rules;
mod test;
mod vm;

use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

use crate::compiler::*;
use crate::vm::*;

fn eval(expr: &str) -> String {
    let tree = parse::parse_tree(expr).expect("expr_to_code: not a tree expression");
    let mut vm = VM::from_expr(tree);
    vm.eval();
    vm.readback().to_string()
}

fn print_help(prog_name: &str) {
    println!("USAGE: {} filename [-c/--compile]", prog_name);
    println!("Reads a tree from `filename`, and evaluates it.\n");
    println!("Flags:");
    println!("-c/--compile    Compile the tree into a .c file instead of interpreting it");
    println!("-h/--help");
}

// Invocation: tc filename [--interpret | --compile]
fn main () {
    // Read command-line args
    let args: Vec<String> = env::args().collect();
    let mut short_flags = Vec::new();
    let mut long_flags = Vec::new();
    let mut filename= None;
    for arg in &args {
        match arg.strip_prefix("-") {
            Some(f) => short_flags.push(f.to_string()),
            None => {
                match arg.strip_prefix("--") {
                    Some(f) => long_flags.push(f.to_string()),
                    None => {
                        filename = Some(arg.to_string());
                    }
                }
            }
        }
    }

    if short_flags.contains(&"h".to_string()) || long_flags.contains(&"help".to_string()) || filename.is_none() {
        print_help(&args[0]);
    } else {
        // Read tree
        let filename_str = filename.unwrap();
        let tree_str = fs::read_to_string(&filename_str).expect("File should be readable");
        if short_flags.contains(&"c".to_string()) || long_flags.contains(&"compile".to_string()) {
            // Compile
            match parse::parse_tree(&tree_str) {
                None => {println!("Error while parseing tree");},
                Some(e) => {
                    let filename_c = filename_str.clone() + ".c";
                    let runtime_c = "src/runtime/runtime.c";
                    let runtime_str = fs::read_to_string(runtime_c).expect("Should be able to read src/runtime/runtime.c");
                    let code_str = compile(&e);

                    let mut file = OpenOptions::new()
                        .create(true)
                        .truncate(true)
                        .write(true)
                        .open(filename_c)
                        .unwrap();
                    file.write(runtime_str.as_bytes()).expect("Should be able to write to file");
                    file.write(code_str.as_bytes()).expect("Should be able to write to file");
                }
            }
        } else {
            // Interpret
            println!("{}", eval(&tree_str));
        }
    }
    // println!("{}", eval("tttt")); // t
    // println!("{}", eval("t(tt)(tt)t")); // tt(ttt)
    // println!("{}", eval("tt(t(t(t(tt(t(t(ttt)(t(tt)))))t(t(tt)(tt)))t)tt)t")); // t(tt)(tt)
    // println!("{}", eval("t t(t(t t)t(t(t t)t)) (t (t t) t)")); // t(t(tt)t)(t(t(tt)t))
    // println!("{}", eval("t (t(t(t(t t t)t))t(t(t t)t)) (t (t t) t)")); // t(t(t(t(tt)t))(t(t(t(tt)t))))(t(tt)t)
    // test_rules();

    // let str = "t(tt)(tt)t";
    // let expr = parse::parse_tree(str).expect("expr_to_code: not a tree expression");
    // println!("{}", compile(&expr));
}
