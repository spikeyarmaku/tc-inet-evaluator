use core::fmt;

use crate::global::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AgentType {L, S, F, E, D, A, T, Q, I}

#[derive(Clone)]
pub struct Port {
    pub agent_addr: HeapAddress,
    pub port_num: PortNum
}

impl Port {
    pub const fn empty() -> Self {
        Self {agent_addr: UNASSIGNED_PORT, port_num: PortNum::Main}
    }

    pub fn new(agent_addr: usize, port_num: PortNum) -> Self {
        Self {agent_addr, port_num}
    }

    // pub fn is_empty(&self) -> bool {
    //     self.agent_addr == UNASSIGNED_PORT
    // }
}

impl fmt::Debug for Port {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.agent_addr != UNASSIGNED_PORT {
            write!(f, "{} - {:?}", self.agent_addr, self.port_num)
        } else {
            write!(f, "")
        }
    }
}

#[derive(Clone, Debug)]
pub struct Agent {
    pub agent_type: AgentType,
    pub ports: [Port; (MAX_AUX_NUM + 1) as usize],
}

impl Agent {
    pub fn new(agent_type: AgentType) -> Self {
        Self {
            agent_type,
            ports: [const { Port::empty() }; (MAX_AUX_NUM + 1) as usize],
        }
    }
}
