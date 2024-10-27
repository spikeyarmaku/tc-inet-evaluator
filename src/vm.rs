use crate::agent::*;
use crate::code::*;
use crate::expr::*;
use crate::global::*;
use crate::rules::*;
use crate::containers::*;

#[derive(PartialEq)]
pub enum EvalState {
    EvalRunning,
    EvalFinished,
}

#[derive(Debug)]
struct Equation {
    pub left_agent: HeapAddress,
    pub right_agent: HeapAddress,
}

// Agents are stored in the heap, and everything else contains indices to
// elements in the heap. The active pairs are pairs of agents connected by their
// principal port
pub struct VM {
    active_pairs: Stack<Equation>,
    heap: Heap<Agent>,
    tape: Tape,

    // left agent max aux num (2) + Right agent max aux num (4) + most agents
    // created in a rule (4) = 10
    reg: [HeapAddress; MAX_AGENT_REG_SIZE as usize],
}

impl VM {
    pub fn from_code(code: Code) -> Self {
        let mut vm = Self {
            active_pairs: Stack::new(),
            heap: Heap::new(),
            tape: Tape::from_code(code),
            reg: [const {UNASSIGNED_PORT}; MAX_AGENT_REG_SIZE as usize],
        };
        vm.exec();
        vm
    }
    // Take an expr, compile it to code, set up the VM, and run the code on it.
    // When it finishes, the expr's inet representation will be loaded in the VM
    pub fn from_expr(expr: Expr) -> Self {
        let code = Code::from_expr(&expr);
        VM::from_code(code)
    }

    // Return true if there are no agents in the heap (only used for debugging)
    pub fn is_empty(&self) -> bool {
        self.heap.len() == 0
    }

    // Execute an interaction rule. Pop the top of the stack until an equation
    // without names is reached. Then set up the registers for both agents, load
    // the code for the appropriate rule, and execute it
    pub fn step(&mut self) -> EvalState {
        // Pop the next equation
        let eq = match self.active_pairs.pop() {
            None => return EvalState::EvalFinished,
            Some(x) => x,
        };
        
        crate::debug_log!("{}\n{}\n{}\n{:?}", self.get_reg(), self.get_heap(),
            self.get_active_pairs(), eq);

        if self.heap[eq.left_agent].agent_type == AgentType::I ||
            self.heap[eq.right_agent].agent_type == AgentType::I
        {
            return EvalState::EvalFinished
        }

        // Set up registers
        self.reg[0] = eq.left_agent;
        for i in 0..MAX_AUX_NUM_LEFT {
            self.reg[(i + 1) as usize] = self.heap[eq.left_agent].ports[i as usize].agent_addr;
        }
        
        self.reg[3] = eq.right_agent;
        for i in 0..MAX_AUX_NUM_RIGHT {
            self.reg[(MAX_AUX_NUM_LEFT + i + 2) as usize] =
                self.heap[eq.right_agent].ports[i as usize].agent_addr;
        }

        // Find the appropriate rule and load its code
        let left_agent_type = self.heap[eq.left_agent].agent_type as u8 - AgentType::L as u8;
        let right_agent_type = self.heap[eq.right_agent].agent_type as u8 - AgentType::E as u8;
        let code_index = left_agent_type * 5 + right_agent_type;
        self.tape.set(Code::from_instrs(RULES[code_index as usize]));
        crate::debug_log!("Invoking rule {}", RULES_NAME[code_index as usize]);

        // Execute the code
        self.exec();

        self.heap.remove(eq.left_agent);
        self.heap.remove(eq.right_agent);

        if self.active_pairs.size() == 0 {
            EvalState::EvalFinished
        } else {
            EvalState::EvalRunning
        }
    }

    // Execute instructions on the tape
    fn exec(&mut self) {
        let mut instr = self.tape.read_instr();
        while instr != Instr::Return {
            self.exec_instr(instr);
            instr = self.tape.read_instr();
        }
    }

    fn connect(&mut self, src_addr: HeapAddress, src_port: PortNum, dst_addr: HeapAddress, dst_port: PortNum) {
        self.heap[src_addr as usize].ports[src_port as usize] =
            Port::new(dst_addr, dst_port);
        self.heap[dst_addr as usize].ports[dst_port as usize] =
            Port::new(src_addr, src_port);
        // If they are connected through their main ports, push them on the stack
        if src_port == PortNum::Main && dst_port == PortNum::Main {
            self.active_pairs.push(Equation {
                left_agent: src_addr,
                right_agent: dst_addr
            });
        }
    }

    // Execute a single instruction
    fn exec_instr(&mut self, instr: Instr) {
        crate::debug_log!("  > {:?}", instr);
        match instr {
            Instr::MkAgent(reg_addr, agent_type) => {
                self.reg[reg_addr as usize] =
                    self.heap.push(Agent::new(agent_type));
            }
            Instr::Connect(src_addr, src_port, dst_addr, dst_port, mode) => {
                let mut real_src_addr = self.reg[src_addr as usize];
                let mut real_src_port = src_port;
                let mut real_dst_addr = self.reg[dst_addr as usize];
                let mut real_dst_port = dst_port;
                if mode == ConnectMode::LeftRef || mode == ConnectMode::FullRef {
                    let agent = &self.heap[real_src_addr];
                    let port = &agent.ports[real_src_port as usize];
                    real_src_addr = port.agent_addr;
                    real_src_port = port.port_num;
                }
                if mode == ConnectMode::RightRef || mode == ConnectMode::FullRef {
                    let agent = &self.heap[real_dst_addr];
                    let port = &agent.ports[real_dst_port as usize];
                    real_dst_addr = port.agent_addr;
                    real_dst_port = port.port_num;
                }
                self.connect(real_src_addr, real_src_port, real_dst_addr, real_dst_port);
            }
            Instr::Load(reg_addr, heap_addr) => {
                self.reg[reg_addr as usize] = heap_addr;
            }
            Instr::Return => return,
        }
    }

    // Evaluate the VM until there are no more active pairs present
    pub fn eval(&mut self) {
        crate::debug_log!("\n=== Evaluating Code ===\n");
        let mut step_count = 0;
        while self.step() == EvalState::EvalRunning {
            step_count += 1;
            crate::debug_log!("\n[ Step {} ]\n", step_count);
        }
        crate::debug_log!("Step count: {}", step_count);
    }

    // Get the register content of the VM as a String (only used for debugging)
    fn get_reg(&self) -> String {
        let mut str = format!("REG - {}:\n", self.reg.len());
        let mut i = 0;
        for e in &self.reg {
            if *e != UNASSIGNED_PORT {
                str.push_str(&format!("  {i}: {:?}\n", e));
            } else {
                str.push_str(&format!("  {i}: {:?}\n", e));
            }
            i += 1;
        }
        str
    }

    // Get the heap content of the VM as a String (only used for debugging)
    fn get_heap(&self) -> String {
        let mut str = format!("HEAP - {} / {}:\n", self.heap.len(), self.heap.full_len());
        let mut i = 0;
        for e in &self.heap {
            str.push_str(&format!("  {i}: {:?}\n", e));
            i += 1;
        }
        str
    }

    // Get the active pairs of the VM as a String (only used for debugging)
    fn get_active_pairs(&self) -> String {
        let mut str = format!("ACTIVE PAIRS - {}:\n", self.active_pairs.size());
        let mut i = 0;
        for e in &self.active_pairs {
            str.push_str(&format!(
                "  {i}: {:?} ({:?} - {:?})\n",
                e, self.heap[e.left_agent].agent_type, self.heap[e.right_agent].agent_type
            ));
            i += 1;
        }
        str
    }

    pub fn readback(&mut self) -> Expr {
        // FIXME It's just a final readback, it assumes the topmost agent is at
        // heap[0]. Since it doesn't handle names, duplication, application,
        // etc., it cannot be used to read back an intermediate state of the VM
        self.readback_agent(0)
    }

    fn readback_agent(&self, agent_addr: HeapAddress) -> Expr {
        let agent = &self.heap[agent_addr];
        match agent.agent_type {
            AgentType::I => {
                let new_addr = agent.ports[0].agent_addr;
                if new_addr == UNASSIGNED_PORT {
                    Expr { children: vec![] }
                } else {
                    self.readback_agent(new_addr)
                }
            }
            AgentType::L => Expr::new(vec![]),
            AgentType::S => Expr::new(vec![self.readback_agent(agent.ports[0].agent_addr)]),
            AgentType::F => Expr::new(vec![
                self.readback_agent(agent.ports[0].agent_addr),
                self.readback_agent(agent.ports[1].agent_addr)]
            ),
            _ => Expr::new(vec![]),
        }
    }
}
