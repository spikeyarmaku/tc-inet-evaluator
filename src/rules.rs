use crate::code::*;
use crate::agent::*;
use crate::global::*;

const fn var(n: RegAddress) -> RegAddress {
    VAR_INDEX_START + n
}

const fn left(n: RegAddress) -> RegAddress {
    n
}

const fn right(n: RegAddress) -> RegAddress {
    MAX_AUX_NUM + n
}

pub const RULES: [&[Instr]; 15] = [
    &RULE_L_E, &RULE_L_D, &RULE_L_A, &RULE_L_T, &RULE_L_Q,
    &RULE_S_E, &RULE_S_D, &RULE_S_A, &RULE_S_T, &RULE_S_Q,
    &RULE_F_E, &RULE_F_D, &RULE_F_A, &RULE_F_T, &RULE_F_Q];

// pub const RULES_NAME: [&str; 15] = [
//     "RULE_L_E", "RULE_L_D", "RULE_L_A", "RULE_L_T", "RULE_L_Q",
//     "RULE_S_E", "RULE_S_D", "RULE_S_A", "RULE_S_T", "RULE_S_Q",
//     "RULE_F_E", "RULE_F_D", "RULE_F_A", "RULE_F_T", "RULE_F_Q"];

pub const RULE_L_E: [Instr; 1] = [
    Instr::Return
];
pub const RULE_L_D: [Instr; 5] = [
    Instr::MkAgent(var(0), AgentType::L),
    Instr::MkAgent(var(1), AgentType::L),
    Instr::Push(var(0), right(0)),
    Instr::Push(var(1), right(1)),
    Instr::Return
];
pub const RULE_L_A: [Instr; 4] = [
    Instr::MkAgent(var(0), AgentType::S),
    Instr::Connect(var(0), 0, right(0)),
    Instr::Push(var(0), right(1)),
    Instr::Return
];
pub const RULE_L_T: [Instr; 4] = [
    Instr::MkAgent(var(0), AgentType::E),
    Instr::Push(right(0), right(2)),
    Instr::Push(right(1), var(0)),
    Instr::Return
];
pub const RULE_L_Q: [Instr; 6] = [
    Instr::MkAgent(var(0), AgentType::E),
    Instr::MkAgent(var(1), AgentType::E),
    Instr::Push(right(0), right(3)),
    Instr::Push(right(1), var(0)),
    Instr::Push(right(2), var(1)),
    Instr::Return
];
pub const RULE_S_E: [Instr; 3] = [
    Instr::MkAgent(var(0), AgentType::E),
    Instr::Push(left(0), var(0)),
    Instr::Return
];
pub const RULE_S_D: [Instr; 13] = [
    Instr::MkAgent(var(0), AgentType::D),
    Instr::MkName(var(1)),
    Instr::MkName(var(2)),
    Instr::Connect(var(0), 0, var(1)),
    Instr::Connect(var(0), 1, var(2)),
    Instr::Push(left(0), var(0)),

    Instr::MkAgent(var(3), AgentType::S),
    Instr::Connect(var(3), 0, var(1)),
    Instr::Push(var(3), right(0)),

    Instr::MkAgent(var(4), AgentType::S),
    Instr::Connect(var(4), 0, var(2)),
    Instr::Push(var(4), right(1)),
    Instr::Return
];
pub const RULE_S_A: [Instr; 5] = [
    Instr::MkAgent(var(0), AgentType::F),
    Instr::Connect(var(0), 0, left(0)),
    Instr::Connect(var(0), 1, right(0)),
    Instr::Push(var(0), right(1)),
    Instr::Return
];
pub const RULE_S_T: [Instr; 19] = [
    Instr::MkAgent(var(0), AgentType::A),
    Instr::MkAgent(var(1), AgentType::A),
    Instr::MkName(var(2)),
    Instr::MkName(var(3)),
    Instr::Push(left(0), var(0)),
    Instr::Connect(var(0), 0, var(2)),
    Instr::Connect(var(0), 1, var(1)),
    Instr::Connect(var(1), 0, var(3)),
    Instr::Connect(var(1), 1, right(2)),

    Instr::MkAgent(var(4), AgentType::D),
    Instr::MkName(var(5)),
    Instr::Push(right(1), var(4)),
    Instr::Connect(var(4), 0, var(5)),
    Instr::Connect(var(4), 1, var(2)),

    Instr::MkAgent(var(6), AgentType::A),
    Instr::Push(right(0), var(6)),
    Instr::Connect(var(6), 0, var(5)),
    Instr::Connect(var(6), 1, var(3)),
    Instr::Return
];
pub const RULE_S_Q: [Instr; 9] = [
    Instr::MkAgent(var(0), AgentType::A),
    Instr::Push(right(1), var(0)),
    Instr::Connect(var(0), 0, left(0)),
    Instr::Connect(var(0), 1, right(3)),

    Instr::MkAgent(var(1), AgentType::E),
    Instr::Push(right(0), var(1)),

    Instr::MkAgent(var(2), AgentType::E),
    Instr::Push(right(2), var(2)),
    Instr::Return
];
pub const RULE_F_E: [Instr; 5] = [
    Instr::MkAgent(var(0), AgentType::E),
    Instr::Push(left(0), var(0)),
    Instr::MkAgent(var(1), AgentType::E),
    Instr::Push(left(1), var(1)),
    Instr::Return
];
pub const RULE_F_D: [Instr; 21] = [
    Instr::MkAgent(var(0), AgentType::F),
    Instr::MkName(var(1)),
    Instr::MkName(var(2)),
    Instr::Connect(var(0), 0, var(1)),
    Instr::Connect(var(0), 1, var(2)),
    Instr::Push(var(0), right(1)),

    Instr::MkAgent(var(3), AgentType::F),
    Instr::MkName(var(4)),
    Instr::MkName(var(5)),
    Instr::Connect(var(3), 0, var(4)),
    Instr::Connect(var(3), 1, var(5)),
    Instr::Push(var(3), right(0)),

    Instr::MkAgent(var(6), AgentType::D),
    Instr::Connect(var(6), 0, var(4)),
    Instr::Connect(var(6), 1, var(1)),
    Instr::Push(left(0), var(6)),

    Instr::MkAgent(var(7), AgentType::D),
    Instr::Connect(var(7), 0, var(5)),
    Instr::Connect(var(7), 1, var(2)),
    Instr::Push(left(1), var(7)),
    Instr::Return
];
pub const RULE_F_A: [Instr; 6] = [
    Instr::MkAgent(var(0), AgentType::T),
    Instr::Connect(var(0), 0, left(1)),
    Instr::Connect(var(0), 1, right(0)),
    Instr::Connect(var(0), 2, right(1)),
    Instr::Push(left(0), var(0)),
    Instr::Return
];
pub const RULE_F_T: [Instr; 7] = [
    Instr::MkAgent(var(0), AgentType::Q),
    Instr::Push(right(1), var(0)),
    Instr::Connect(var(0), 0, left(0)),
    Instr::Connect(var(0), 1, left(1)),
    Instr::Connect(var(0), 2, right(0)),
    Instr::Connect(var(0), 3, right(2)),
    Instr::Return
];
pub const RULE_F_Q: [Instr; 12] = [
    Instr::MkAgent(var(0), AgentType::A),
    Instr::MkAgent(var(1), AgentType::A),
    Instr::Push(right(3), var(0)),
    Instr::Connect(var(0), 0, left(0)),
    Instr::Connect(var(0), 1, var(1)),
    Instr::Connect(var(1), 0, left(1)),
    Instr::Connect(var(1), 1, right(3)),

    Instr::MkAgent(var(2), AgentType::E),
    Instr::Push(right(0), var(2)),

    Instr::MkAgent(var(3), AgentType::E),
    Instr::Push(right(1), var(3)),
    Instr::Return
];
