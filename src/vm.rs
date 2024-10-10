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
    reg: [HeapAddress; MAX_REG_SIZE as usize], // `as` is usually frowned upon, but it works with consts
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

    pub fn push(&mut self, item: T) -> usize {
        match self.data.iter().position(|r| r.is_none()) {
            None => {
                self.data.push(Some(item));
                self.data.len() - 1
            }
            Some(i) => {
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
        let code = expr_to_code(&expr);
        let mut vm = Self {active_pairs: Stack::new(), heap: Heap::new(), reg: [const {0}; MAX_REG_SIZE as usize], tape: Tape::from_code(code)};
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
                    if agent0.ports[0] == 0 {
                        let agent0 = self.heap.get_mut(eq.left_agent);
                        agent0.ports[0] = eq.right_agent;
                    } else {
                        self.active_pairs.push(Equation{left_agent: agent0.ports[0], right_agent: eq.right_agent});
                        self.heap.remove(eq.left_agent);
                    }
                }
            } else {
                if agent1.ports[0] == 0 {
                    let agent1 = &mut self.heap.get_mut(eq.right_agent);
                    agent1.ports[0] = eq.left_agent;
                } else {
                    self.active_pairs.push(Equation{left_agent: eq.left_agent, right_agent: agent1.ports[0]});
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
            self.reg[i + MAX_AUX_NUM as usize] = self.heap.get(eq.right_agent).ports[i];
        }

        // Load the instructions for this pair
        let code_index =
            (self.heap.get(eq.left_agent).agent_type as u8 - AgentType::L as u8) * 5 +
            self.heap.get(eq.right_agent).agent_type as u8 - AgentType::E as u8;
        // {Name, L, S, F, E, D, A, T, Q}
        self.tape.set(RULES[code_index as usize].to_vec());
        // println!("Invoking rule {}", RULES_NAME[code_index as usize]);

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
        // println!("{:?}", instr);
        match instr {
            Instr::MkAgent(reg_addr, agent_type) => {
                self.reg[reg_addr as usize] = self.heap.push(Agent::new(agent_type));
            }
            Instr::MkName(reg_addr) => {
                let heap_addr = self.heap.len();
                self.heap.push(Agent::new(AgentType::Name));
                self.reg[reg_addr as usize] = heap_addr;
            }
            Instr::Connect(src_reg_addr, port_num, dst_reg_addr) => {
                self.heap.get_mut(self.reg[src_reg_addr as usize]).ports[port_num as usize] = self.reg[dst_reg_addr as usize];
            }
            Instr::Push(reg_addr0, reg_addr1) => {
                self.active_pairs.push(Equation{left_agent: self.reg[reg_addr0 as usize], right_agent: self.reg[reg_addr1 as usize]});
            }
            // Instr::Store(reg_addr, heap_addr) => {}
            Instr::Load(heap_addr, reg_addr) => {
                self.reg[reg_addr as usize] = heap_addr;
            }
            Instr::Return => return
        }
    }

    pub fn eval(&mut self) {
        // self.print_heap();
        // self.print_active_pairs();
        let mut step_count = 0;
        while self.step() == EvalState::EvalRunning {
            step_count += 1;
            // println!("Step count: {:?}", step_count);
            // self.print_heap();
            // self.print_active_pairs();
            // if step_count % 10000 == 0 {
            // }
        }
        println!("Step count: {}", step_count);
    }

    pub fn print_stats(&self) {
        println!("Heap item count: {}", self.heap.len());
        println!("Active pair count: {}", self.active_pairs.size());
    }

    pub fn print_heap(&self) {
        println!("HEAP: ");
        let mut i = 0;
        for e in &self.heap.data {
            println!("{i}: {:?}", e);
            i += 1;
        }
        println!("END OF HEAP");
    }

    pub fn print_active_pairs(&self) {
        println!("ACTIVE PAIRS");
        let mut i = 0;
        for e in &self.active_pairs.stack {
            println!("{i}: {:?}", e);
            i += 1;
        }
        println!("END OF ACTIVE PAIRS");
    }

    pub fn readback(&mut self) -> Expr {
        // TODO it's just a final readback, it assumes the topmost equation is the Tree <-> result one
        match self.active_pairs.pop() {
            None => Expr::make_leaf(),
            Some(eq) => self.readback_agent(eq.left_agent)
        }
    }
    
    fn readback_agent(&self, agent_addr: HeapAddress) -> Expr {
        let agent = self.heap.get(agent_addr);
        match agent.agent_type {
            AgentType::Name => {
                let new_addr = agent.ports[0];
                if new_addr == 0 {
                    Expr {children: vec![]}
                } else {
                    self.readback_agent(new_addr)
                }
            },
            AgentType::L => Expr::make_leaf(),
            AgentType::S => Expr::make_stem(self.readback_agent(agent.ports[0])),
            AgentType::F => Expr::make_fork(self.readback_agent(agent.ports[0]), self.readback_agent(agent.ports[1])),
            _ => Expr::make_leaf()
        }
    }
}
