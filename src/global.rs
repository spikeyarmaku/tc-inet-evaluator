pub const MAX_AUX_NUM: u8 = 4;
// MAX_AUX_NUM * 2 + max number of agents created in a rewrite rule
pub const MAX_REG_SIZE: u8 = MAX_AUX_NUM * 2 + 7; // This should probably be a macro
pub const VAR_INDEX_START: u8 = MAX_AUX_NUM * 2;

pub type RegAddress = u8;
pub type HeapAddress = usize; // 0 is always reserved for result, so it is an invalid value for ports
pub type PortNum = u8;