// Given an expr, compile it to LLVM IR

use crate::expr::*;

pub struct TargetConfig {
    pub address_size: u8
}

impl TargetConfig {
    pub fn default() -> Self {
        Self {address_size: size_of::<usize>() as u8}
    }
}

pub fn compile(expr: &Expr, conf: TargetConfig) -> String {
    "".to_string()
}
