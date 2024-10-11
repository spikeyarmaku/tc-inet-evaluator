pub const DEBUG: bool = false;

pub const MAX_AUX_NUM: u8 = 4;
// MAX_AUX_NUM * 2 + max number of agents created in a rewrite rule
pub const MAX_REG_SIZE: u8 = MAX_AUX_NUM * 2 + 8; // This should be a macro?
pub const VAR_INDEX_START: u8 = MAX_AUX_NUM * 2;
pub const UNASSIGNED_PORT: HeapAddress = HeapAddress::MAX;

pub type RegAddress = u8;
pub type HeapAddress = usize;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PortNum {P0, P1, P2, P3}

// https://stackoverflow.com/questions/74586162/how-to-import-use-macro-from-different-module-in-the-same-crate
#[macro_export]
macro_rules! debug_log {
    ($($args: tt)*) => {
        if DEBUG {
            println!($($args)*);
        }
    }
}
