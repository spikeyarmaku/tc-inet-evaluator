use std::fmt;
use std::fmt::Display;
use std::collections::HashMap;

use crate::expr::*;
use crate::code::*;
use crate::global::*;

impl Display for PortNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortNum::P0 => {write!(f, "P0")},
            PortNum::P1 => {write!(f, "P1")},
            PortNum::P2 => {write!(f, "P2")},
            PortNum::P3 => {write!(f, "P3")},
            PortNum::Main => {write!(f, "PMAIN")},
        }
    }
}

impl Display for ConnectMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectMode::NoRef => {write!(f, "NO_REF")},
            ConnectMode::LeftRef => {write!(f, "SRC_REF")},
            ConnectMode::RightRef => {write!(f, "DST_REF")},
            ConnectMode::FullRef => {write!(f, "FULL_REF")},
        }
    }
}

pub fn compile(expr: &Expr) -> String {
    let code = Code::from_expr(expr);
    let mut agent_index = 0;
    let mut code_str = "void init_tree() {\n".to_string();
    let mut reg = HashMap::new();
    for c in code {
        match c {
            Instr::MkAgent(_, agent_type) => {
                code_str += &format!("    struct Agent* agent{agent_index} = mk_agent(AGENT_{:?});\n", agent_type);
                agent_index += 1;
            },
            Instr::Connect(reg_addr0, port0, reg_addr1, port1, conn_mode) => {
                let agent0 = reg.get(&reg_addr0).unwrap();
                let agent1 = reg.get(&reg_addr1).unwrap();
                code_str += &format!("    connect({agent0}, {}, {agent1}, {}, {});\n", port0, port1, conn_mode);
            },
            Instr::Load(reg_addr, heap_addr) => {
                reg.insert(reg_addr, format!("agent{heap_addr}"));
            },
            Instr::Return => {}
        }
    }
    code_str += "}\n\nint main() {\n    run();\n}\n";
    code_str
}
