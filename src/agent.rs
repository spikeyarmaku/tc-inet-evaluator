use crate::global::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AgentType {Name, L, S, F, E, D, A, T, Q}

#[derive(Clone, Debug)]
pub struct Agent {
    pub agent_type: AgentType,
    pub ports: [HeapAddress; MAX_AUX_NUM as usize],
}

impl Agent {
    pub fn new(agent_type: AgentType) -> Self {
        Agent {agent_type, ports: [const {UNASSIGNED_PORT}; 4]}
    }
}