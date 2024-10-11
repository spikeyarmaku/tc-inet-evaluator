use crate::agent::*;
use crate::code::*;
use crate::expr::*;
use crate::global::*;
use crate::rules::*;

#[derive(PartialEq)]
pub enum EvalState {EvalRunning, EvalFinished}

struct Stack<T> {
    stack: Vec<T>,
}

#[derive(Debug)]
struct Equation {
    pub left_agent: HeapAddress,
    pub right_agent: HeapAddress,
}

pub struct VM {
    // word_size: u8,
    active_pairs: Stack<Equation>,
    heap: Heap<Agent>,
    // `as` is usually frowned upon, but it works with consts
    reg: [HeapAddress; MAX_REG_SIZE as usize],
    tape: Tape,
}

struct Heap<T> {
    empty_count: usize,
    data: Vec<Option<T>>
}

impl<T> Heap<T> {
    pub fn new() -> Self {
        Heap{empty_count: 0, data: Vec::new()}
    }

    pub fn len(&self) -> usize {
        self.data.len() - self.empty_count
    }

    pub fn full_len(&self) -> usize {
        self.data.len()
    }

    // Find the first empty position (value = None), put the element there, and
    // return the position. If no empty positions found, append it to the end of
    // the list
    pub fn push(&mut self, item: T) -> usize {
        match self.data.iter().position(|r| r.is_none()) {
            None => {
                self.data.push(Some(item));
                self.data.len() - 1
            }
            Some(i) => {
                // crate::debug_log!("Push to {i}");
                self.data[i] = Some(item);
                self.empty_count -= 1;
                i
            }
        }
    }

    pub fn remove(&mut self, index: usize) {
        self.data[index] = None;
        self.empty_count += 1;
        self.shrink();
    }

    pub fn get(&self, index: usize) -> &T {
        self.data[index].as_ref().unwrap()
    }

    pub fn get_mut(&mut self, index: usize) -> &mut T {
        self.data[index].as_mut().unwrap()
    }

    // Delete superfluous `None`s from the end
    fn shrink(&mut self) {
        while self.data.last().unwrap().is_none() {
            self.data.pop();
            self.empty_count -= 1;
        }
    }
}

// push, pop, peek, size
impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {stack: Vec::new()}
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }
}

impl VM {
    pub fn from_expr(expr: Expr) -> Self {
        crate::debug_log!("\n=== STEP 2: Compiling Expr to Code ===\n");
        let code = expr_to_code(&expr);
        let mut vm = Self {
            active_pairs: Stack::new(),
            heap: Heap::new(),
            reg: [const {UNASSIGNED_PORT}; MAX_REG_SIZE as usize],
            tape: Tape::from_code(code)};
        vm.exec();
        vm
    }

    fn resolve_names(&mut self) {
        loop {
            let eq: Equation = match self.active_pairs.pop() {
                None => return,
                Some(x) => x
            };
            let agent0 = self.heap.get(eq.left_agent);
            let agent1 = self.heap.get(eq.right_agent);
            
            if agent1.agent_type != AgentType::Name {
                if agent0.agent_type != AgentType::Name {
                    self.active_pairs.push(eq);
                    return
                } else {
                    if agent0.ports[0] == UNASSIGNED_PORT {
                        let agent0 =
                            self.heap.get_mut(eq.left_agent);
                        agent0.ports[0] = eq.right_agent;
                    } else {
                        self.active_pairs.push(Equation{
                            left_agent: agent0.ports[0],
                            right_agent: eq.right_agent});
                        self.heap.remove(eq.left_agent);
                    }
                }
            } else {
                if agent1.ports[0] == UNASSIGNED_PORT {
                    let agent1 =
                        &mut self.heap.get_mut(eq.right_agent);
                    agent1.ports[0] = eq.left_agent;
                } else {
                    self.active_pairs.push(Equation{
                        left_agent: eq.left_agent,
                        right_agent: agent1.ports[0]});
                    self.heap.remove(eq.right_agent);
                }
            }
        }
    }

    pub fn step(&mut self) -> EvalState {
        self.resolve_names();
        let eq = match self.active_pairs.pop() {
            None => return EvalState::EvalFinished,
            Some(x) => x
        };

        // Set up registers
        for i in 0..MAX_AUX_NUM as usize {
            self.reg[i] = self.heap.get(eq.left_agent).ports[i];
            self.reg[i + MAX_AUX_NUM as usize] =
                self.heap.get(eq.right_agent).ports[i];
        }

        crate::debug_log!("{}\n{}\n{}\n{:?}", self.get_reg(), self.get_heap(),
            self.get_active_pairs(), eq);

        let left_agent_type =
            self.heap.get(eq.left_agent).agent_type as u8 -
            AgentType::L as u8;
        let right_agent_type =
            self.heap.get(eq.right_agent).agent_type as u8 -
            AgentType::E as u8;
        let code_index = left_agent_type * 5 + right_agent_type;
        self.tape.set(RULES[code_index as usize].to_vec());
        crate::debug_log!("Invoking rule {}", RULES_NAME[code_index as usize]);

        self.exec();
        self.heap.remove(eq.left_agent);
        self.heap.remove(eq.right_agent);

        if self.active_pairs.size() == 0 {
            EvalState::EvalFinished
        } else {
            EvalState::EvalRunning
        }
    }

    fn exec(&mut self) {
        let mut instr = self.tape.read_instr();
        while instr != Instr::Return {
            self.exec_instr(instr);
            instr = self.tape.read_instr();
        }
    }

    fn exec_instr(&mut self, instr: Instr) {
        crate::debug_log!("  > {:?}", instr);
        match instr {
            Instr::MkAgent(reg_addr, agent_type) => {
                self.reg[reg_addr as usize] =
                    self.heap.push(Agent::new(agent_type));
            }
            Instr::Connect(src_reg_addr, port_num, dst_reg_addr) => {
                self.heap.get_mut(self.reg[src_reg_addr as usize])
                    .ports[port_num as usize] = self.reg[dst_reg_addr as usize];
            }
            Instr::Push(reg_addr0, reg_addr1) => {
                self.active_pairs.push(Equation{
                    left_agent: self.reg[reg_addr0 as usize],
                    right_agent: self.reg[reg_addr1 as usize]});
            }
            // Instr::Store(reg_addr, heap_addr) => {}
            Instr::Load(heap_addr, reg_addr) => {
                self.reg[reg_addr as usize] = heap_addr;
            }
            Instr::Return => return
        }
    }

    pub fn eval(&mut self) {
        crate::debug_log!("\n=== STEP 3: Evaluating Code ===\n");
        let mut step_count = 0;
        while self.step() == EvalState::EvalRunning {
            step_count += 1;
            crate::debug_log!("\n[ Step {} ]\n", step_count);
        }
        crate::debug_log!("Step count: {}", step_count);
    }

    fn get_reg(&self) -> String {
        let mut str = format!("REG - {}:\n", self.reg.len());
        let mut i = 0;
        for e in &self.reg {
            str.push_str(&format!("  {i}: {:?}\n", e));
            i += 1;
        }
        str
    }

    fn get_heap(&self) -> String {
        let mut str = format!("HEAP - {} / {}:\n", self.heap.len(),
            self.heap.full_len());
        let mut i = 0;
        for e in &self.heap.data {
            str.push_str(&format!("  {i}: {:?}\n", e));
            i += 1;
        }
        str
    }

    fn get_active_pairs(&self) -> String {
        let mut str = format!("ACTIVE PAIRS - {}:\n",
            self.active_pairs.size());
        let mut i = 0;
        for e in &self.active_pairs.stack {
            str.push_str(&format!("  {i}: {:?} ({:?} - {:?})\n", e,
                self.heap.get(e.left_agent).agent_type,
                self.heap.get(e.right_agent).agent_type));
            i += 1;
        }
        str
    }

    pub fn readback(&mut self) -> Expr {
        // TODO it's just a final readback, it assumes the topmost equation is
        // the Tree <-> result one
        // match self.active_pairs.pop() {
        //     None => Expr::make_leaf(),
        //     Some(eq) => self.readback_agent(eq.left_agent)
        // }
        self.readback_agent(0)
    }
    
    fn readback_agent(&self, agent_addr: HeapAddress) -> Expr {
        let agent = self.heap.get(agent_addr);
        match agent.agent_type {
            AgentType::Name => {
                let new_addr = agent.ports[0];
                if new_addr == UNASSIGNED_PORT {
                    Expr {children: vec![]}
                } else {
                    self.readback_agent(new_addr)
                }
            },
            AgentType::L => Expr::make_leaf(),
            AgentType::S => Expr::make_stem(
                self.readback_agent(agent.ports[0])),
            AgentType::F => Expr::make_fork(
                self.readback_agent(agent.ports[0]),
                self.readback_agent(agent.ports[1])),
            _ => Expr::make_leaf()
        }
    }
}
