use crate::global::*;
use crate::agent::*;
use crate::expr::*;

pub type Code = Vec<Instr>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instr {
    MkAgent(RegAddress, AgentType),
    Connect(RegAddress, PortNum, RegAddress),
    Push(RegAddress, RegAddress),
    Load(HeapAddress, RegAddress),
    Return
}

pub struct Tape {
    pc: HeapAddress,
    code: Code
}

// set, read byte, read word
impl Tape {
    pub fn from_code(new_code: Code) -> Self {
        Tape {pc: 0, code: new_code}
    }

    pub fn set(&mut self, code: Code) {
        self.code = code;
        self.pc = 0;
    }

    pub fn read_instr(&mut self) -> Instr {
        let result: Instr = self.code[self.pc];
        self.pc += 1;
        result
    }
}

fn program_to_code(expr: &Expr, next_id: HeapAddress) -> (HeapAddress, Code) {
    match expr.children.len() {
        0 => {
            (next_id, vec![
                Instr::MkAgent(0, AgentType::L),
            ])
        }
        1 => {
            // Create instructions for the subtree
            let (child_addr, mut code) =
                program_to_code(&expr.children[0], next_id);
            code.append(&mut vec![
                Instr::MkAgent(0, AgentType::S),
                Instr::Load(child_addr, 1),
                Instr::Connect(0, 0, 1),
            ]);
            (child_addr + 1, code)
        }
        _ => {
            let (child0_addr, mut code) =
                program_to_code(&expr.children[0], next_id);
            let (child1_addr, mut right_code) =
                program_to_code(&expr.children[1],
                    child0_addr + 1);
            code.append(&mut right_code);
            code.append(&mut vec![
                Instr::Load(child0_addr, 1),
                Instr::Load(child1_addr, 2),
                Instr::MkAgent(0, AgentType::F),
                Instr::Connect(0, 0, 1),
                Instr::Connect(0, 1, 2),
            ]);
            (child1_addr + 1, code)
        }
    }
}

fn application_to_code(expr: &Expr, next_id: HeapAddress) -> (HeapAddress, Code) {
    let rightmost_child = &expr.children[expr.children.len() - 1];
    let left_children = Expr{
        children: expr.children[0..expr.children.len() - 1].to_vec()};
    let (child0_addr, mut child0_code) =
        expr_to_code_and_pos(&left_children, next_id);
    let (child1_addr, mut child1_code) =
        expr_to_code_and_pos(&rightmost_child, child0_addr + 1);
    child0_code.append(&mut child1_code);
    child0_code.append(&mut vec![
        Instr::MkAgent(0, AgentType::A),
        Instr::Load(child0_addr, 1),
        Instr::Load(child1_addr, 2)
    ]);
    match rightmost_child.get_type() {
        ExprType::Program => {
            child0_code.append(&mut vec![Instr::Connect(0, 0, 2)]);
        }
        ExprType::Application => {
            child0_code.append(&mut vec![
                Instr::MkAgent(3, AgentType::Name),
                Instr::Connect(0, 0, 3),
                Instr::Connect(2, 1, 3)
            ]);
        }
    }
    match left_children.get_type() {
        ExprType::Program => {
            child0_code.append(&mut vec![Instr::Push(1, 0)]);
        }
        ExprType::Application => {
            child0_code.append(&mut vec![Instr::Connect(1, 1, 0)]);
        }
    }
    (child1_addr + 1, child0_code)
}

fn expr_to_code_and_pos(expr: &Expr, next_id: HeapAddress) -> (HeapAddress, Code) {
    match expr.get_type() {
        ExprType::Program => {
            program_to_code(expr, next_id)
        }
        ExprType::Application => {
            application_to_code(expr, next_id)
        }
    }
}

pub fn expr_to_code(expr: &Expr) -> Code {
    // Interface
    let mut code: Code = vec![
        Instr::MkAgent(0, AgentType::Name),
    ];
    match expr.get_type() {
        ExprType::Program => {
            let (addr, mut tree_code) =
                program_to_code(&expr, 1);
            code.append(&mut tree_code);
            // Otherwise, push the root node and the interface
            code.append(&mut vec![
                Instr::Load(0, 0),
                Instr::Load(addr, 1),
                Instr::Push(0, 1)]);
        }
        ExprType::Application => {
            let (addr, mut tree_code) =
                application_to_code(&expr, 1);
            code.append(&mut tree_code);
            // If the tree is an application, connect its p1 to the interface
            code.append(&mut vec![
                Instr::Load(0, 0),
                Instr::Load(addr, 1),
                Instr::Connect(1, 1, 0)]);
        }
    }
    code.append(&mut vec![Instr::Return]);
    code
}
