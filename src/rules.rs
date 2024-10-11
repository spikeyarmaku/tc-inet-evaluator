use crate::agent::*;
use crate::code::*;
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
    &RULE_L_E, &RULE_L_D, &RULE_L_A, &RULE_L_T, &RULE_L_Q, &RULE_S_E, &RULE_S_D, &RULE_S_A,
    &RULE_S_T, &RULE_S_Q, &RULE_F_E, &RULE_F_D, &RULE_F_A, &RULE_F_T, &RULE_F_Q,
];

pub const RULES_NAME: [&str; 15] = [
    "RULE_L_E", "RULE_L_D", "RULE_L_A", "RULE_L_T", "RULE_L_Q", "RULE_S_E", "RULE_S_D", "RULE_S_A",
    "RULE_S_T", "RULE_S_Q", "RULE_F_E", "RULE_F_D", "RULE_F_A", "RULE_F_T", "RULE_F_Q",
];

// L >< E => ;
pub const RULE_L_E: [Instr; 1] = [Instr::Return];
// L >< D(x, y) => L~x, L~y;
pub const RULE_L_D: [Instr; 5] = [
    Instr::MkAgent(var(0), AgentType::L),
    Instr::MkAgent(var(1), AgentType::L),
    Instr::Push(var(0), right(0)),
    Instr::Push(var(1), right(1)),
    Instr::Return,
];
// L >< A(x, r) => S(x)~r;
pub const RULE_L_A: [Instr; 4] = [
    Instr::MkAgent(var(0), AgentType::S),
    Instr::Connect(var(0), PortNum::P0, right(0)),
    Instr::Push(var(0), right(1)),
    Instr::Return,
];
// L >< T(x, y, r) => x~r, y~E;
pub const RULE_L_T: [Instr; 4] = [
    Instr::MkAgent(var(0), AgentType::E),
    Instr::Push(right(0), right(2)),
    Instr::Push(right(1), var(0)),
    Instr::Return,
];
// L >< Q(x, y, z, r) => x~r, y~E, z~E;
pub const RULE_L_Q: [Instr; 6] = [
    Instr::MkAgent(var(0), AgentType::E),
    Instr::MkAgent(var(1), AgentType::E),
    Instr::Push(right(0), right(3)),
    Instr::Push(right(1), var(0)),
    Instr::Push(right(2), var(1)),
    Instr::Return,
];
// S(m) >< E => m~E;
pub const RULE_S_E: [Instr; 3] = [
    Instr::MkAgent(var(0), AgentType::E),
    Instr::Push(left(0), var(0)),
    Instr::Return,
];
// S(m) >< D(x, y) => m~D(a, b), S(a)~x, S(b)~y;
pub const RULE_S_D: [Instr; 13] = [
    Instr::MkAgent(var(0), AgentType::D),
    Instr::MkAgent(var(1), AgentType::Name),
    Instr::MkAgent(var(2), AgentType::Name),
    Instr::Connect(var(0), PortNum::P0, var(1)),
    Instr::Connect(var(0), PortNum::P1, var(2)),
    Instr::Push(left(0), var(0)),
    Instr::MkAgent(var(3), AgentType::S),
    Instr::Connect(var(3), PortNum::P0, var(1)),
    Instr::Push(var(3), right(0)),
    Instr::MkAgent(var(4), AgentType::S),
    Instr::Connect(var(4), PortNum::P0, var(2)),
    Instr::Push(var(4), right(1)),
    Instr::Return,
];
// S(m) >< A(x, r) => F(m, x)~r;
pub const RULE_S_A: [Instr; 5] = [
    Instr::MkAgent(var(0), AgentType::F),
    Instr::Connect(var(0), PortNum::P0, left(0)),
    Instr::Connect(var(0), PortNum::P1, right(0)),
    Instr::Push(var(0), right(1)),
    Instr::Return,
];
// S(m) >< T(x, y, r) => m~A(b, A(c, r)), y~D(a, b), x~A(a, c);
pub const RULE_S_T: [Instr; 19] = [
    Instr::MkAgent(var(0), AgentType::A),
    Instr::MkAgent(var(1), AgentType::A),
    Instr::MkAgent(var(2), AgentType::Name), // b
    Instr::MkAgent(var(3), AgentType::Name), // c
    Instr::Push(left(0), var(0)),
    Instr::Connect(var(0), PortNum::P0, var(2)),
    Instr::Connect(var(0), PortNum::P1, var(1)),
    Instr::Connect(var(1), PortNum::P0, var(3)),
    Instr::Connect(var(1), PortNum::P1, right(2)),
    Instr::MkAgent(var(4), AgentType::D),
    Instr::MkAgent(var(5), AgentType::Name), // a
    Instr::Push(right(1), var(4)),
    Instr::Connect(var(4), PortNum::P0, var(5)),
    Instr::Connect(var(4), PortNum::P1, var(2)),
    Instr::MkAgent(var(6), AgentType::A),
    Instr::Push(right(0), var(6)),
    Instr::Connect(var(6), PortNum::P0, var(5)),
    Instr::Connect(var(6), PortNum::P1, var(3)),
    Instr::Return,
];
// S(m) >< Q(x, y, z, r) => y~A(m, r), x~E, z~E;
pub const RULE_S_Q: [Instr; 9] = [
    Instr::MkAgent(var(0), AgentType::A),
    Instr::Push(right(1), var(0)),
    Instr::Connect(var(0), PortNum::P0, left(0)),
    Instr::Connect(var(0), PortNum::P1, right(3)),
    Instr::MkAgent(var(1), AgentType::E),
    Instr::Push(right(0), var(1)),
    Instr::MkAgent(var(2), AgentType::E),
    Instr::Push(right(2), var(2)),
    Instr::Return,
];
// F(m, n) >< E => m~E, n~E;
pub const RULE_F_E: [Instr; 5] = [
    Instr::MkAgent(var(0), AgentType::E),
    Instr::Push(left(0), var(0)),
    Instr::MkAgent(var(1), AgentType::E),
    Instr::Push(left(1), var(1)),
    Instr::Return,
];
// F(m, n) >< D(x, y) => F(a, b)~x, F(c, d)~y, m~D(a, c), n~D(b, d);
pub const RULE_F_D: [Instr; 21] = [
    Instr::MkAgent(var(0), AgentType::F),
    Instr::MkAgent(var(1), AgentType::Name),
    Instr::MkAgent(var(2), AgentType::Name),
    Instr::Connect(var(0), PortNum::P0, var(1)),
    Instr::Connect(var(0), PortNum::P1, var(2)),
    Instr::Push(var(0), right(1)),
    Instr::MkAgent(var(3), AgentType::F),
    Instr::MkAgent(var(4), AgentType::Name),
    Instr::MkAgent(var(5), AgentType::Name),
    Instr::Connect(var(3), PortNum::P0, var(4)),
    Instr::Connect(var(3), PortNum::P1, var(5)),
    Instr::Push(var(3), right(0)),
    Instr::MkAgent(var(6), AgentType::D),
    Instr::Connect(var(6), PortNum::P0, var(4)),
    Instr::Connect(var(6), PortNum::P1, var(1)),
    Instr::Push(left(0), var(6)),
    Instr::MkAgent(var(7), AgentType::D),
    Instr::Connect(var(7), PortNum::P0, var(5)),
    Instr::Connect(var(7), PortNum::P1, var(2)),
    Instr::Push(left(1), var(7)),
    Instr::Return,
];
// F(m, n) >< A(x, r) => m~T(n, x, r);
pub const RULE_F_A: [Instr; 6] = [
    Instr::MkAgent(var(0), AgentType::T),
    Instr::Connect(var(0), PortNum::P0, left(1)),
    Instr::Connect(var(0), PortNum::P1, right(0)),
    Instr::Connect(var(0), PortNum::P2, right(1)),
    Instr::Push(left(0), var(0)),
    Instr::Return,
];
// F(m, n) >< T(x, y, r) => y~Q(m, n, x, r);
pub const RULE_F_T: [Instr; 7] = [
    Instr::MkAgent(var(0), AgentType::Q),
    Instr::Push(right(1), var(0)),
    Instr::Connect(var(0), PortNum::P0, left(0)),
    Instr::Connect(var(0), PortNum::P1, left(1)),
    Instr::Connect(var(0), PortNum::P2, right(0)),
    Instr::Connect(var(0), PortNum::P3, right(2)),
    Instr::Return,
];
// F(m, n) >< Q(x, y, z, r) => z~A(m, A(n, r)), x~E, y~E;
pub const RULE_F_Q: [Instr; 12] = [
    Instr::MkAgent(var(0), AgentType::A),
    Instr::MkAgent(var(1), AgentType::A),
    Instr::Push(right(2), var(0)),
    Instr::Connect(var(0), PortNum::P0, left(0)),
    Instr::Connect(var(0), PortNum::P1, var(1)),
    Instr::Connect(var(1), PortNum::P0, left(1)),
    Instr::Connect(var(1), PortNum::P1, right(3)),
    Instr::MkAgent(var(2), AgentType::E),
    Instr::Push(right(0), var(2)),
    Instr::MkAgent(var(3), AgentType::E),
    Instr::Push(right(1), var(3)),
    Instr::Return,
];
