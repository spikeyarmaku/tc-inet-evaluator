pub const DEBUG: bool = false;

pub const MAX_AUX_NUM: u8 = 4;
// MAX_AUX_NUM * 2 + max number of agents created in a rewrite rule
pub const MAX_REG_SIZE: u8 = MAX_AUX_NUM * 2 + 8; // This should be a macro?
pub const VAR_INDEX_START: u8 = MAX_AUX_NUM * 2;
pub const UNASSIGNED_PORT: usize = usize::MAX;

pub type RegAddress = u8;
// 0 is always reserved for result, so it is an invalid value for ports
pub type HeapAddress = usize;
pub type PortNum = u8;

// https://stackoverflow.com/questions/74586162/how-to-import-use-macro-from-different-module-in-the-same-crate
#[macro_export]
macro_rules! debug_log {
    ($($args: tt)*) => {
        if DEBUG {
            println!($($args)*);
        }
    }
}
