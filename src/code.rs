use std::ops::Index;

use crate::agent::*;
use crate::expr::*;
use crate::global::*;

pub struct Code(Vec<Instr>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConnectMode {NoRef, LeftRef, RightRef, FullRef}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instr {
    // Create an agent and store it on the heap
    MkAgent     (RegAddress, AgentType),
    // Connect two existing connections
    Connect     (RegAddress, PortNum, RegAddress, PortNum, ConnectMode),
    Load        (RegAddress, HeapAddress),
    // End marker of a series of instructions
    Return,
}

pub struct Tape {
    pc: usize,
    code: Code,
}

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
        // for i in instrs {
        //     println!("+ {:?}", i);
        // }
        self.0.extend(instrs);
    }

    // Given a heap address and an expr, compile the creation of the expr (as a
    // chain of L's and A's) at the given address to instructions, and return
    // the next free heap address
    fn expr_to_code(&mut self, expr: &Expr, heap_addr: HeapAddress) -> HeapAddress {
        match expr.children.len() {
            0 => {
                self.record_instrs(&[Instr::MkAgent(0, AgentType::L)]);
                heap_addr + 1
            },
            1 => {
                self.record_instrs(&[Instr::MkAgent(0, AgentType::S)]);
                let heap_addr_1 = heap_addr + 1;
                let heap_addr_2 = self.expr_to_code(&expr.children[0], heap_addr_1);
                self.record_instrs(&[
                    Instr::Load(0, heap_addr),
                    Instr::Load(1, heap_addr_1),
                ]);
                if expr.children[0].children.len() < 3 {
                    self.record_instrs(&[Instr::Connect(0, PortNum::P0, 1, PortNum::Main, ConnectMode::NoRef)]);
                } else {
                    self.record_instrs(&[Instr::Connect(0, PortNum::P0, 1, PortNum::P1, ConnectMode::NoRef)]);
                }
                heap_addr_2
            },
            2 => {
                self.record_instrs(&[Instr::MkAgent(0, AgentType::F)]);
                let heap_addr_1 = heap_addr + 1;
                let heap_addr_2 = self.expr_to_code(&expr.children[0], heap_addr_1);
                let heap_addr_3 = self.expr_to_code(&expr.children[1], heap_addr_2);
                self.record_instrs(&[
                    Instr::Load(0, heap_addr),
                    Instr::Load(1, heap_addr_1),
                    Instr::Load(2, heap_addr_2),
                ]);
                if expr.children[0].children.len() < 3 {
                    self.record_instrs(&[Instr::Connect(0, PortNum::P0, 1, PortNum::Main, ConnectMode::NoRef)]);
                } else {
                    self.record_instrs(&[Instr::Connect(0, PortNum::P0, 1, PortNum::P1, ConnectMode::NoRef)]);
                }
                if expr.children[1].children.len() < 3 {
                    self.record_instrs(&[Instr::Connect(0, PortNum::P1, 2, PortNum::Main, ConnectMode::NoRef)]);
                } else {
                    self.record_instrs(&[Instr::Connect(0, PortNum::P1, 2, PortNum::P1, ConnectMode::NoRef)]);
                }
                heap_addr_3
            }
            _ => {
                // println!("\nn children");
                println!("{} children found", expr.children.len());
                self.record_instrs(&[Instr::MkAgent(0, AgentType::A)]);
                let heap_addr_1 = heap_addr + 1;
                let (last_child, rest) = expr.children.split_last().unwrap();
                let new_expr = Expr{children: rest.to_vec()};
                let heap_addr_2 = self.expr_to_code(&new_expr, heap_addr_1);
                let heap_addr_3 = self.expr_to_code(last_child, heap_addr_2);
                self.record_instrs(&[
                    Instr::Load(0, heap_addr), // A
                    Instr::Load(1, heap_addr_1), // left
                    Instr::Load(2, heap_addr_2), // right
                ]);
                if new_expr.children.len() < 3 {
                    self.record_instrs(&[Instr::Connect(1, PortNum::Main, 0, PortNum::Main, ConnectMode::NoRef)]);
                } else {
                    self.record_instrs(&[Instr::Connect(0, PortNum::Main, 1, PortNum::P1, ConnectMode::NoRef)]);
                }
                if last_child.children.len() < 3 {
                    self.record_instrs(&[Instr::Connect(0, PortNum::P0, 2, PortNum::Main, ConnectMode::NoRef)]);
                } else {
                    self.record_instrs(&[Instr::Connect(0, PortNum::P0, 2, PortNum::P1, ConnectMode::NoRef)]);
                }
                heap_addr_3
            }
        }
    }

    pub fn from_instrs(instrs: &[Instr]) -> Self {
        Self(instrs.to_vec())
    }

    // Compile a tree expression to code. Only use L and A nodes
    pub fn from_expr(expr: &Expr) -> Self {
        // Create interface at heap[0]. At the end, this will be used to read
        // back the result
        let mut code = Self::from_instrs(&[Instr::MkAgent(0, AgentType::I)]);

        // Compile the expr into code
        code.expr_to_code(&expr, 1);

        code.record_instrs(&[
            Instr::Load(0, 0),
            Instr::Load(1, 1),
        ]);

        // Connect the two
        if expr.children.len() < 3 {
            // If `expr` is just a node, push it and the interface
            code.record_instrs(&[Instr::Connect(0, PortNum::P0, 1, PortNum::Main, ConnectMode::NoRef)]);
        } else {
            // If `expr` is an application, connect its p1 to the interface
            code.record_instrs(&[Instr::Connect(1, PortNum::P1, 0, PortNum::P0, ConnectMode::NoRef)]);
        }

        // Return
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

// This trait is generated by ChatGPT
impl IntoIterator for Code {
    type Item = Instr;
    type IntoIter = std::vec::IntoIter<Instr>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Code {
    type Item = &'a Instr;
    type IntoIter = std::slice::Iter<'a, Instr>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Code {
    type Item = &'a mut Instr;
    type IntoIter = std::slice::IterMut<'a, Instr>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
