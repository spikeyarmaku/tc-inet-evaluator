use std::ops::Index;

use crate::agent::*;
use crate::expr::*;
use crate::global::*;

pub struct Code(Vec<Instr>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instr {
    // Create an agent and store it on the heap
    MkAgent (RegAddress,    AgentType               ),
    // Connect a port of an agent to the principal port of another agent. To
    // connect two aux ports, an intermediate Name agent has to be used
    Connect (RegAddress,    PortNum,    RegAddress  ),
    // Connect two agents by their principal ports and add them to the active
    // pair stack
    Push    (RegAddress,    RegAddress              ),
    // Set up the registers according to an agent on the heap
    Load    (HeapAddress,   RegAddress              ),
    // End marker of a series of instructions
    Return,
}

pub struct Tape {
    pc: usize,
    code: Code,
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

impl Code {
    fn record_instrs(&mut self, instrs: &[Instr]) {
        self.0.extend(instrs);
    }

    fn program_to_code(&mut self, expr: &Expr, next_id: HeapAddress) -> HeapAddress {
        match expr.children.len() {
            0 => {
                self.record_instrs(&[Instr::MkAgent(0, AgentType::L)]);
                next_id
            }
            1 => {
                // Create instructions for the subtree
                let child_addr = self.program_to_code(&expr.children[0], next_id);
                self.record_instrs(&[
                    Instr::MkAgent(0, AgentType::S),
                    Instr::Load(child_addr, 1),
                    Instr::Connect(0, PortNum::P0, 1),
                ]);
                child_addr + 1
            }
            _ => {
                let child0_addr = self.program_to_code(&expr.children[0], next_id);
                let child1_addr = self.program_to_code(&expr.children[1], child0_addr + 1);
                self.record_instrs(&[
                    Instr::Load(child0_addr, 1),
                    Instr::Load(child1_addr, 2),
                    Instr::MkAgent(0, AgentType::F),
                    Instr::Connect(0, PortNum::P0, 1),
                    Instr::Connect(0, PortNum::P1, 2),
                ]);
                child1_addr + 1
            }
        }
    }

    fn application_to_code(&mut self, expr: &Expr, next_id: HeapAddress) -> HeapAddress {
        let rightmost_child = &expr.children[expr.children.len() - 1];
        let left_children = Expr {
            children: expr.children[0..expr.children.len() - 1].to_vec(),
        };
        let child0_addr = self.expr_to_code(&left_children, next_id);
        let child1_addr = self.expr_to_code(&rightmost_child, child0_addr + 1);
        self.record_instrs(&[
            Instr::MkAgent(0, AgentType::A),
            Instr::Load(child0_addr, 1),
            Instr::Load(child1_addr, 2),
        ]);
        match rightmost_child.get_type() {
            ExprType::Program => {
                self.record_instrs(&[Instr::Connect(0, PortNum::P0, 2)]);
            }
            ExprType::Application => {
                self.record_instrs(&[
                    Instr::MkAgent(3, AgentType::Name),
                    Instr::Connect(0, PortNum::P0, 3),
                    Instr::Connect(2, PortNum::P1, 3),
                ]);
            }
        }
        match left_children.get_type() {
            ExprType::Program => self.record_instrs(&[Instr::Push(1, 0)]),
            ExprType::Application => self.record_instrs(&[Instr::Connect(1, PortNum::P1, 0)])
        }
        child1_addr + 1
    }

    fn expr_to_code(&mut self, expr: &Expr, next_id: HeapAddress) -> HeapAddress {
        match expr.get_type() {
            ExprType::Program => self.program_to_code(expr, next_id),
            ExprType::Application => self.application_to_code(expr, next_id),
        }
    }

    pub fn from_instrs(instrs: &[Instr]) -> Self {
        Self(instrs.to_vec())
    }

    // FIXME It doesn't handle cases like t(tttt)
    // Compile a tree expression to code. Automatically use L, S and F agents
    // and only put A where a node has more than two children
    pub fn from_expr(expr: &Expr) -> Self {
        // Create interface at heap[0]
        let mut code = Self::from_instrs(&[Instr::MkAgent(0, AgentType::Name)]);

        // Compile the expr to code
        match expr.get_type() {
            ExprType::Program => {
                let addr = code.program_to_code(&expr, 1);
                // If `expr` is a program, push the root node and the interface
                code.record_instrs(&[
                    Instr::Load(0, 0),
                    Instr::Load(addr, 1),
                    Instr::Push(0, 1)
                ]);
            }
            ExprType::Application => {
                let addr = code.application_to_code(&expr, 1);
                // If `expr` is an application, connect its p1 to the interface
                code.record_instrs(&[
                    Instr::Load(0, 0),
                    Instr::Load(addr, 1),
                    Instr::Connect(1, PortNum::P1, 0),
                ]);
            }
        }
        code.record_instrs(&[Instr::Return]);
        code
    }
}

impl Index<usize> for Code {
    type Output = Instr;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
