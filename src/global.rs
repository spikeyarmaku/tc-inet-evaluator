pub const DEBUG: bool = true;

// maximum number of ports an agent can have
pub const MAX_AUX_NUM: u8 = 4;
pub const MAX_AUX_NUM_LEFT: u8 = 2;
pub const MAX_AUX_NUM_RIGHT: u8 = 4;
pub const MAX_AGENTS_CREATED: u8 = 4;
pub const MAX_AGENT_REG_SIZE: u8 = MAX_AUX_NUM_LEFT + MAX_AUX_NUM_RIGHT + MAX_AGENTS_CREATED + 2;
pub const MAX_PORT_REG_SIZE: u8 = MAX_AUX_NUM_LEFT + MAX_AUX_NUM_RIGHT;
pub const UNASSIGNED_PORT: HeapAddress = HeapAddress::MAX;

pub type RegAddress = u8;
pub type HeapAddress = usize;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PortNum {P0, P1, P2, P3, Main}

// https://stackoverflow.com/questions/74586162/how-to-import-use-macro-from-different-module-in-the-same-crate
#[macro_export]
macro_rules! debug_log {
    ($($args: tt)*) => {
        if DEBUG {
            println!($($args)*);
        }
    }
}
