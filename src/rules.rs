// New implementation:
// Compilation of Interaction Nets - Abubakar Hassan, Ian Mackie, Shinya Sato
// https://core.ac.uk/download/pdf/82756233.pdf

use crate::agent::*;
use crate::code::*;
use crate::global::*;

const fn left_agent() -> u8 {
    0
}

const fn right_agent() -> u8 {
    3
}

const fn var(n: u8) -> u8 {
    n + 8
}

pub const RULES: [&[Instr]; 15] = [
    &RULE_L_E, &RULE_L_D, &RULE_L_A, &RULE_L_T, &RULE_L_Q,
    &RULE_S_E, &RULE_S_D, &RULE_S_A, &RULE_S_T, &RULE_S_Q,
    &RULE_F_E, &RULE_F_D, &RULE_F_A, &RULE_F_T, &RULE_F_Q,
];

pub const RULES_NAME: [&str; 15] = [
    "RULE_L_E", "RULE_L_D", "RULE_L_A", "RULE_L_T", "RULE_L_Q",
    "RULE_S_E", "RULE_S_D", "RULE_S_A", "RULE_S_T", "RULE_S_Q",
    "RULE_F_E", "RULE_F_D", "RULE_F_A", "RULE_F_T", "RULE_F_Q",
];

// L >< E => ;
pub const RULE_L_E: [Instr; 1] = [
    Instr::Return
];

// L >< D(x, y) => L~x, L~y;
pub const RULE_L_D: [Instr; 5] = [
    Instr::MkAgent(var(0), AgentType::L),
    Instr::MkAgent(var(1), AgentType::L),
    Instr::Connect(var(0), PortNum::Main, right_agent(), PortNum::P0, ConnectMode::RightRef),
    Instr::Connect(var(1), PortNum::Main, right_agent(), PortNum::P1, ConnectMode::RightRef),
    Instr::Return,
];

// L >< A(x, r) => S(x)~r;
pub const RULE_L_A: [Instr; 4] = [
    Instr::MkAgent(var(0), AgentType::S),
    Instr::Connect(var(0), PortNum::Main, right_agent(), PortNum::P1, ConnectMode::RightRef),
    Instr::Connect(var(0), PortNum::P0, right_agent(), PortNum::P0, ConnectMode::RightRef),
    Instr::Return,
];

// L >< T(x, y, r) => x~r, y~E;
pub const RULE_L_T: [Instr; 4] = [
    Instr::MkAgent(var(0), AgentType::E),
    Instr::Connect(right_agent(), PortNum::P1, var(0), PortNum::Main, ConnectMode::LeftRef),
    Instr::Connect(right_agent(), PortNum::P0, right_agent(), PortNum::P2, ConnectMode::FullRef),
    Instr::Return,
];

// L >< Q(x, y, z, r) => x~r, y~E, z~E;
pub const RULE_L_Q: [Instr; 6] = [
    Instr::MkAgent(var(0), AgentType::E),
    Instr::MkAgent(var(1), AgentType::E),
    Instr::Connect(right_agent(), PortNum::P0, right_agent(), PortNum::P3, ConnectMode::FullRef),
    Instr::Connect(right_agent(), PortNum::P1, var(0), PortNum::Main, ConnectMode::LeftRef),
    Instr::Connect(right_agent(), PortNum::P2, var(1), PortNum::Main, ConnectMode::LeftRef),
    Instr::Return,
];

// S(m) >< E => m~E;
pub const RULE_S_E: [Instr; 3] = [
    Instr::MkAgent(var(0), AgentType::E),
    Instr::Connect(left_agent(), PortNum::P0, var(0), PortNum::Main, ConnectMode::LeftRef),
    Instr::Return,
];

// S(m) >< D(x, y) => m~D(a, b), S(a)~x, S(b)~y;
pub const RULE_S_D: [Instr; 9] = [
    Instr::MkAgent(var(0), AgentType::D),
    Instr::MkAgent(var(1), AgentType::S),
    Instr::MkAgent(var(2), AgentType::S),
    Instr::Connect(var(0), PortNum::P0, var(1), PortNum::P0, ConnectMode::NoRef),
    Instr::Connect(var(0), PortNum::P1, var(2), PortNum::P0, ConnectMode::NoRef),
    Instr::Connect(left_agent(), PortNum::P0, var(0), PortNum::Main, ConnectMode::LeftRef),
    Instr::Connect(var(1), PortNum::Main, right_agent(), PortNum::P0, ConnectMode::RightRef),
    Instr::Connect(var(2), PortNum::Main, right_agent(), PortNum::P1, ConnectMode::RightRef),
    Instr::Return,
];

// S(m) >< A(x, r) => F(m, x)~r;
pub const RULE_S_A: [Instr; 5] = [
    Instr::MkAgent(var(0), AgentType::F),
    Instr::Connect(var(0), PortNum::P0, left_agent(), PortNum::P0, ConnectMode::RightRef),
    Instr::Connect(var(0), PortNum::P1, right_agent(), PortNum::P0, ConnectMode::RightRef),
    Instr::Connect(var(0), PortNum::Main, right_agent(), PortNum::P1, ConnectMode::RightRef),
    Instr::Return,
];

// S(m) >< T(x, y, r) => m~A(b, A(c, r)), y~D(a, b), x~A(a, c);
pub const RULE_S_T: [Instr; 13] = [
    Instr::MkAgent(var(0), AgentType::A),
    Instr::MkAgent(var(1), AgentType::A),
    Instr::MkAgent(var(2), AgentType::D),
    Instr::MkAgent(var(3), AgentType::A),
    Instr::Connect(var(0), PortNum::P0, var(2), PortNum::P0, ConnectMode::NoRef),
    Instr::Connect(var(0), PortNum::P1, var(1), PortNum::P0, ConnectMode::NoRef),
    Instr::Connect(var(1), PortNum::Main, var(3), PortNum::P1, ConnectMode::NoRef),
    Instr::Connect(var(2), PortNum::P1, var(3), PortNum::P0, ConnectMode::NoRef),
    Instr::Connect(right_agent(), PortNum::P0, var(0), PortNum::Main, ConnectMode::LeftRef),
    Instr::Connect(right_agent(), PortNum::P2, var(1), PortNum::P1, ConnectMode::LeftRef),
    Instr::Connect(right_agent(), PortNum::P1, var(2), PortNum::Main, ConnectMode::LeftRef),
    Instr::Connect(left_agent(), PortNum::P0, var(3), PortNum::Main, ConnectMode::LeftRef),
    Instr::Return,
];

// S(m) >< Q(x, y, z, r) => y~A(m, r), x~E, z~E;
pub const RULE_S_Q: [Instr; 9] = [
    Instr::MkAgent(var(0), AgentType::A),
    Instr::MkAgent(var(1), AgentType::E),
    Instr::MkAgent(var(2), AgentType::E),
    Instr::Connect(var(0), PortNum::P0, left_agent(), PortNum::P0, ConnectMode::RightRef),
    Instr::Connect(var(0), PortNum::P1, right_agent(), PortNum::P3, ConnectMode::RightRef),
    Instr::Connect(right_agent(), PortNum::P1, var(0), PortNum::Main, ConnectMode::LeftRef),
    Instr::Connect(right_agent(), PortNum::P0, var(1), PortNum::Main, ConnectMode::LeftRef),
    Instr::Connect(right_agent(), PortNum::P2, var(2), PortNum::Main, ConnectMode::LeftRef),
    Instr::Return,
];

// F(m, n) >< E => m~E, n~E;
pub const RULE_F_E: [Instr; 5] = [
    Instr::MkAgent(var(0), AgentType::E),
    Instr::MkAgent(var(1), AgentType::E),
    Instr::Connect(left_agent(), PortNum::P0, var(0), PortNum::Main, ConnectMode::LeftRef),
    Instr::Connect(left_agent(), PortNum::P1, var(1), PortNum::Main, ConnectMode::LeftRef),
    Instr::Return,
];

// F(m, n) >< D(x, y) => F(a, b)~x, F(c, d)~y, m~D(a, c), n~D(b, d);
pub const RULE_F_D: [Instr; 13] = [
    Instr::MkAgent(var(0), AgentType::F),
    Instr::MkAgent(var(1), AgentType::F),
    Instr::MkAgent(var(2), AgentType::D),
    Instr::MkAgent(var(3), AgentType::D),
    Instr::Connect(var(0), PortNum::P0, var(2), PortNum::P0, ConnectMode::NoRef),
    Instr::Connect(var(0), PortNum::P1, var(3), PortNum::P0, ConnectMode::NoRef),
    Instr::Connect(var(1), PortNum::P0, var(2), PortNum::P1, ConnectMode::NoRef),
    Instr::Connect(var(1), PortNum::P1, var(3), PortNum::P1, ConnectMode::NoRef),
    Instr::Connect(var(0), PortNum::Main, right_agent(), PortNum::P0, ConnectMode::RightRef),
    Instr::Connect(var(1), PortNum::Main, right_agent(), PortNum::P1, ConnectMode::RightRef),
    Instr::Connect(left_agent(), PortNum::P0, var(2), PortNum::Main, ConnectMode::LeftRef),
    Instr::Connect(left_agent(), PortNum::P1, var(3), PortNum::Main, ConnectMode::LeftRef),
    Instr::Return,
];

// F(m, n) >< A(x, r) => m~T(n, x, r);
pub const RULE_F_A: [Instr; 6] = [
    Instr::MkAgent(var(0), AgentType::T),
    Instr::Connect(var(0), PortNum::P0, left_agent(), PortNum::P1, ConnectMode::RightRef),
    Instr::Connect(var(0), PortNum::P1, right_agent(), PortNum::P0, ConnectMode::RightRef),
    Instr::Connect(var(0), PortNum::P2, right_agent(), PortNum::P1, ConnectMode::RightRef),
    Instr::Connect(left_agent(), PortNum::P0, var(0), PortNum::Main, ConnectMode::LeftRef),
    Instr::Return,
];

// F(m, n) >< T(x, y, r) => y~Q(m, n, x, r);
pub const RULE_F_T: [Instr; 7] = [
    Instr::MkAgent(var(0), AgentType::Q),
    Instr::Connect(var(0), PortNum::P0, left_agent(), PortNum::P0, ConnectMode::RightRef),
    Instr::Connect(var(0), PortNum::P1, left_agent(), PortNum::P1, ConnectMode::RightRef),
    Instr::Connect(var(0), PortNum::P2, right_agent(), PortNum::P0, ConnectMode::RightRef),
    Instr::Connect(var(0), PortNum::P3, right_agent(), PortNum::P2, ConnectMode::RightRef),
    Instr::Connect(right_agent(), PortNum::P1, var(0), PortNum::Main, ConnectMode::LeftRef),
    Instr::Return,
];

// F(m, n) >< Q(x, y, z, r) => z~A(m, A(n, r)), x~E, y~E;
pub const RULE_F_Q: [Instr; 12] = [
    Instr::MkAgent(var(0), AgentType::A),
    Instr::MkAgent(var(1), AgentType::A),
    Instr::MkAgent(var(2), AgentType::E),
    Instr::MkAgent(var(3), AgentType::E),
    Instr::Connect(var(0), PortNum::P1, var(1), PortNum::Main, ConnectMode::NoRef),
    Instr::Connect(var(0), PortNum::P0, left_agent(), PortNum::P0, ConnectMode::RightRef),
    Instr::Connect(var(1), PortNum::P0, left_agent(), PortNum::P1, ConnectMode::RightRef),
    Instr::Connect(var(1), PortNum::P1, right_agent(), PortNum::P3, ConnectMode::RightRef),
    Instr::Connect(right_agent(), PortNum::P2, var(0), PortNum::Main, ConnectMode::LeftRef),
    Instr::Connect(right_agent(), PortNum::P0, var(2), PortNum::Main, ConnectMode::LeftRef),
    Instr::Connect(right_agent(), PortNum::P1, var(3), PortNum::Main, ConnectMode::LeftRef),
    Instr::Return,
];
