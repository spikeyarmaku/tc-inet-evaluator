use std::ops::Index;
use std::thread::current;

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
    // Set the register's value to the address
    Load    (RegAddress,    HeapAddress             ),
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

    // Given a heap address and an expr, compile the creation of the expr (as a
    // chain of L's and A's) at the given address to instructions, and return
    // the next free heap address
    fn expr_to_code(&mut self, expr: &Expr, heap_addr: HeapAddress) -> HeapAddress {
        if expr.children.is_empty() {
            self.record_instrs(&[Instr::MkAgent(0, AgentType::L)]);
            heap_addr + 1
        } else {
            self.record_instrs(&[
                Instr::MkAgent(0, AgentType::A),
                Instr::MkAgent(1, AgentType::L),
            ]);
            
            let mut addr_child0 = heap_addr + 1;
            let mut next_free_addr = heap_addr + 2;
            for i in 0..expr.children.len() {
                let c = &expr.children[i];
                let addr_child1 = next_free_addr;
                next_free_addr = self.expr_to_code(c, addr_child1);
                let a_addr = next_free_addr;
                
                // Create the A node, except for the last one, since that has
                // already been created at the beginning (because that is what
                // had to be created at the provided address)
                if i < expr.children.len() - 1 {
                    self.record_instrs(&[Instr::MkAgent(0, AgentType::A)]);
                    next_free_addr += 1;
                } else {
                    self.record_instrs(&[Instr::Load(0, heap_addr)]);
                }

                // Load the children
                self.record_instrs(&[
                    Instr::Load(1, addr_child0),
                    Instr::Load(2, addr_child1),
                ]);

                // Connect child0 and the A node
                if i == 0 {
                    // At first, child0 is an L node
                    self.record_instrs(&[Instr::Push(1, 0)]);
                } else {
                    // Later, child0 is an A node
                    self.record_instrs(&[Instr::Connect(1, PortNum::P1, 0)]);
                }

                // Connect child1 and the A node
                if c.children.is_empty() {
                    // If child1 has no children, it is an L node
                    self.record_instrs(&[Instr::Connect(0, PortNum::P0, 2)]);
                } else {
                    // Else it is an A node and a name has to be created
                    self.record_instrs(&[
                        Instr::MkAgent(3, AgentType::Name),
                        Instr::Connect(0, PortNum::P0, 3),
                        Instr::Connect(2, PortNum::P1, 3),
                    ]);
                    next_free_addr += 1;
                }

                addr_child0 = a_addr;
            }
            next_free_addr
        }
    }

    pub fn from_instrs(instrs: &[Instr]) -> Self {
        Self(instrs.to_vec())
    }

    // Compile a tree expression to code. Automatically use L, S and F agents
    // and only put A where a node has more than two children
    pub fn from_expr(expr: &Expr) -> Self {
        // Create interface at heap[0]
        let mut code = Self::from_instrs(&[Instr::MkAgent(0, AgentType::Name)]);

        code.expr_to_code(&expr, 1);
        code.record_instrs(&[
            Instr::Load(0, 0),
            Instr::Load(1, 1),
        ]);
        if expr.children.is_empty() {
            // If `expr` is just a node, push it and the interface
            code.record_instrs(&[Instr::Push(0, 1)]);
        } else {
            // If `expr` is an application, connect its p1 to the interface
            code.record_instrs(&[Instr::Connect(1, PortNum::P1, 0)]);
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
