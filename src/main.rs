mod agent;
mod code;
mod compiler;
mod containers;
mod expr;
mod global;
mod parse;
mod rules;
mod vm;

use crate::agent::*;
use crate::code::*;
use crate::global::*;
use crate::vm::*;

fn eval(expr: &str) -> String {
    let tree = parse::parse_tree(expr).expect("expr_to_code: not a tree expression");
    let mut vm = VM::from_expr(tree);
    vm.eval();
    vm.readback().to_string()
}

// pub const RULES: [&[Instr]; 15] = [
//     &RULE_L_E, &RULE_L_D, &RULE_L_A, &RULE_L_T, &RULE_L_Q,
//     &RULE_S_E, &RULE_S_D, &RULE_S_A, &RULE_S_T, &RULE_S_Q,
//     &RULE_F_E, &RULE_F_D, &RULE_F_A, &RULE_F_T, &RULE_F_Q,
// ];

fn test_rules() {
    test_rule("L-E", Code::from_instrs(&[
        Instr::MkAgent(0, AgentType::L),
        Instr::MkAgent(1, AgentType::E),
        Instr::Connect(0, PortNum::Main, 1, PortNum::Main, ConnectMode::NoRef),
        Instr::Return,
    ]));

    test_rule("S-E", Code::from_instrs(&[
        Instr::MkAgent(0, AgentType::S),
        Instr::MkAgent(1, AgentType::E),
        Instr::MkAgent(2, AgentType::L),
        Instr::Connect(0, PortNum::Main, 1, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(2, PortNum::Main, 0, PortNum::P0, ConnectMode::NoRef),
        Instr::Return,
    ]));

    test_rule("F-E", Code::from_instrs(&[
        Instr::MkAgent(0, AgentType::F),
        Instr::MkAgent(1, AgentType::E),
        Instr::MkAgent(2, AgentType::L),
        Instr::MkAgent(3, AgentType::L),
        Instr::Connect(0, PortNum::Main, 1, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(2, PortNum::Main, 0, PortNum::P0, ConnectMode::NoRef),
        Instr::Connect(3, PortNum::Main, 0, PortNum::P1, ConnectMode::NoRef),
        Instr::Return,
    ]));

    test_rule("L-D", Code::from_instrs(&[
        Instr::MkAgent(0, AgentType::L),
        Instr::MkAgent(1, AgentType::D),
        Instr::MkAgent(2, AgentType::E),
        Instr::MkAgent(3, AgentType::E),
        Instr::Connect(0, PortNum::Main, 1, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(1, PortNum::P0, 2, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(1, PortNum::P1, 3, PortNum::Main, ConnectMode::NoRef),
        Instr::Return,
    ]));

    test_rule("S-D", Code::from_instrs(&[
        Instr::MkAgent(0, AgentType::S),
        Instr::MkAgent(1, AgentType::D),
        Instr::MkAgent(2, AgentType::E),
        Instr::MkAgent(3, AgentType::E),
        Instr::MkAgent(4, AgentType::L),
        Instr::Connect(0, PortNum::Main, 1, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(1, PortNum::P0, 2, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(1, PortNum::P1, 3, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(4, PortNum::Main, 0, PortNum::P0, ConnectMode::NoRef),
        Instr::Return,
    ]));

    test_rule("F-D", Code::from_instrs(&[
        Instr::MkAgent(0, AgentType::F),
        Instr::MkAgent(1, AgentType::D),
        Instr::MkAgent(2, AgentType::E),
        Instr::MkAgent(3, AgentType::E),
        Instr::MkAgent(4, AgentType::L),
        Instr::MkAgent(5, AgentType::L),
        Instr::Connect(0, PortNum::Main, 1, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(1, PortNum::P0, 2, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(1, PortNum::P1, 3, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(4, PortNum::Main, 0, PortNum::P0, ConnectMode::NoRef),
        Instr::Connect(5, PortNum::Main, 0, PortNum::P1, ConnectMode::NoRef),
        Instr::Return,
    ]));

    test_rule("L-A", Code::from_instrs(&[
        Instr::MkAgent(0, AgentType::L),
        Instr::MkAgent(1, AgentType::A),
        Instr::MkAgent(2, AgentType::L),
        Instr::MkAgent(3, AgentType::E),
        Instr::Connect(0, PortNum::Main, 1, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(2, PortNum::Main, 1, PortNum::P0, ConnectMode::NoRef),
        Instr::Connect(3, PortNum::Main, 1, PortNum::P1, ConnectMode::NoRef),
        Instr::Return,
    ]));

    test_rule("S-A", Code::from_instrs(&[
        Instr::MkAgent(0, AgentType::S),
        Instr::MkAgent(1, AgentType::A),
        Instr::MkAgent(2, AgentType::L),
        Instr::MkAgent(3, AgentType::E),
        Instr::MkAgent(4, AgentType::L),
        Instr::Connect(0, PortNum::Main, 1, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(2, PortNum::Main, 1, PortNum::P0, ConnectMode::NoRef),
        Instr::Connect(3, PortNum::Main, 1, PortNum::P1, ConnectMode::NoRef),
        Instr::Connect(4, PortNum::Main, 0, PortNum::P0, ConnectMode::NoRef),
        Instr::Return,
    ]));

    test_rule("F-A", Code::from_instrs(&[
        Instr::MkAgent(0, AgentType::F),
        Instr::MkAgent(1, AgentType::A),
        Instr::MkAgent(2, AgentType::L),
        Instr::MkAgent(3, AgentType::E),
        Instr::MkAgent(4, AgentType::L),
        Instr::MkAgent(5, AgentType::L),
        Instr::Connect(0, PortNum::Main, 1, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(2, PortNum::Main, 1, PortNum::P0, ConnectMode::NoRef),
        Instr::Connect(3, PortNum::Main, 1, PortNum::P1, ConnectMode::NoRef),
        Instr::Connect(4, PortNum::Main, 0, PortNum::P0, ConnectMode::NoRef),
        Instr::Connect(5, PortNum::Main, 0, PortNum::P1, ConnectMode::NoRef),
        Instr::Return,
    ]));

    test_rule("L-T", Code::from_instrs(&[
        Instr::MkAgent(0, AgentType::L),
        Instr::MkAgent(1, AgentType::T),
        Instr::MkAgent(2, AgentType::L),
        Instr::MkAgent(3, AgentType::L),
        Instr::MkAgent(4, AgentType::E),
        Instr::Connect(0, PortNum::Main, 1, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(2, PortNum::Main, 1, PortNum::P0, ConnectMode::NoRef),
        Instr::Connect(3, PortNum::Main, 1, PortNum::P1, ConnectMode::NoRef),
        Instr::Connect(4, PortNum::Main, 1, PortNum::P2, ConnectMode::NoRef),
        Instr::Return,
    ]));

    test_rule("S-T", Code::from_instrs(&[
        Instr::MkAgent(0, AgentType::S),
        Instr::MkAgent(1, AgentType::T),
        Instr::MkAgent(2, AgentType::L),
        Instr::MkAgent(3, AgentType::L),
        Instr::MkAgent(4, AgentType::E),
        Instr::MkAgent(5, AgentType::L),
        Instr::Connect(0, PortNum::Main, 1, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(2, PortNum::Main, 1, PortNum::P0, ConnectMode::NoRef),
        Instr::Connect(3, PortNum::Main, 1, PortNum::P1, ConnectMode::NoRef),
        Instr::Connect(4, PortNum::Main, 1, PortNum::P2, ConnectMode::NoRef),
        Instr::Connect(5, PortNum::Main, 0, PortNum::P0, ConnectMode::NoRef),
        Instr::Return,
    ]));

    test_rule("F-T", Code::from_instrs(&[
        Instr::MkAgent(0, AgentType::F),
        Instr::MkAgent(1, AgentType::T),
        Instr::MkAgent(2, AgentType::L),
        Instr::MkAgent(3, AgentType::L),
        Instr::MkAgent(4, AgentType::E),
        Instr::MkAgent(5, AgentType::L),
        Instr::MkAgent(6, AgentType::L),
        Instr::Connect(0, PortNum::Main, 1, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(2, PortNum::Main, 1, PortNum::P0, ConnectMode::NoRef),
        Instr::Connect(3, PortNum::Main, 1, PortNum::P1, ConnectMode::NoRef),
        Instr::Connect(4, PortNum::Main, 1, PortNum::P2, ConnectMode::NoRef),
        Instr::Connect(5, PortNum::Main, 0, PortNum::P0, ConnectMode::NoRef),
        Instr::Connect(6, PortNum::Main, 0, PortNum::P1, ConnectMode::NoRef),
        Instr::Return,
    ]));

    test_rule("L-Q", Code::from_instrs(&[
        Instr::MkAgent(0, AgentType::L),
        Instr::MkAgent(1, AgentType::Q),
        Instr::MkAgent(2, AgentType::L),
        Instr::MkAgent(3, AgentType::L),
        Instr::MkAgent(4, AgentType::L),
        Instr::MkAgent(5, AgentType::E),
        Instr::Connect(0, PortNum::Main, 1, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(2, PortNum::Main, 1, PortNum::P0, ConnectMode::NoRef),
        Instr::Connect(3, PortNum::Main, 1, PortNum::P1, ConnectMode::NoRef),
        Instr::Connect(4, PortNum::Main, 1, PortNum::P2, ConnectMode::NoRef),
        Instr::Connect(5, PortNum::Main, 1, PortNum::P3, ConnectMode::NoRef),
        Instr::Return,
    ]));

    test_rule("S-Q", Code::from_instrs(&[
        Instr::MkAgent(0, AgentType::S),
        Instr::MkAgent(1, AgentType::Q),
        Instr::MkAgent(2, AgentType::L),
        Instr::MkAgent(3, AgentType::L),
        Instr::MkAgent(4, AgentType::L),
        Instr::MkAgent(5, AgentType::E),
        Instr::MkAgent(6, AgentType::L),
        Instr::Connect(0, PortNum::Main, 1, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(2, PortNum::Main, 1, PortNum::P0, ConnectMode::NoRef),
        Instr::Connect(3, PortNum::Main, 1, PortNum::P1, ConnectMode::NoRef),
        Instr::Connect(4, PortNum::Main, 1, PortNum::P2, ConnectMode::NoRef),
        Instr::Connect(5, PortNum::Main, 1, PortNum::P3, ConnectMode::NoRef),
        Instr::Connect(6, PortNum::Main, 0, PortNum::P0, ConnectMode::NoRef),
        Instr::Return,
    ]));

    test_rule("F-Q", Code::from_instrs(&[
        Instr::MkAgent(0, AgentType::F),
        Instr::MkAgent(1, AgentType::Q),
        Instr::MkAgent(2, AgentType::L),
        Instr::MkAgent(3, AgentType::L),
        Instr::MkAgent(4, AgentType::L),
        Instr::MkAgent(5, AgentType::E),
        Instr::MkAgent(6, AgentType::L),
        Instr::MkAgent(7, AgentType::L),
        Instr::Connect(0, PortNum::Main, 1, PortNum::Main, ConnectMode::NoRef),
        Instr::Connect(2, PortNum::Main, 1, PortNum::P0, ConnectMode::NoRef),
        Instr::Connect(3, PortNum::Main, 1, PortNum::P1, ConnectMode::NoRef),
        Instr::Connect(4, PortNum::Main, 1, PortNum::P2, ConnectMode::NoRef),
        Instr::Connect(5, PortNum::Main, 1, PortNum::P3, ConnectMode::NoRef),
        Instr::Connect(6, PortNum::Main, 0, PortNum::P0, ConnectMode::NoRef),
        Instr::Connect(7, PortNum::Main, 0, PortNum::P1, ConnectMode::NoRef),
        Instr::Return,
    ]));
}

fn test_rule(rule_name: &str, code: Code) {
    crate::debug_log!("\n >>> Testing rule {} <<< \n", rule_name);
    let mut vm = VM::from_code(code);
    vm.eval();
    if !vm.is_empty() {
        crate::debug_log!("INCORRECT");
        std::process::exit(1);
    }
}

fn main () {
    // println!("{}", eval("tttt")); // t
    // println!("{}", eval("t(tt)(tt)t")); // tt(ttt)
    // println!("{}", eval("tt(t(t(t(tt(t(t(ttt)(t(tt)))))t(t(tt)(tt)))t)tt)t")); // t(tt)(tt)
    // println!("{}", eval("t t(t(t t)t(t(t t)t)) (t (t t) t)")); // t(t(tt)t)(t(t(tt)t))
    println!("{}", eval("t (t(t(t(t t t)t))t(t(t t)t)) (t (t t) t)")); // t(t(t(t(tt)t))(t(t(t(tt)t))))(t(tt)t)
    // test_rules();

    // let str = "t(tt)(tt)t";
    // let expr = parse::parse_tree(str).expect("expr_to_code: not a tree expression");
    // println!("{}", compile(&expr));
}