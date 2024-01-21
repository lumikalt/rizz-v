use std::collections::HashMap;

use crate::{err::RuntimeErr, parser::{Loc, Token}};

#[derive(Debug)]
pub struct Env {
    pub register_alias: HashMap<String, u32>,
    labels: HashMap<String, u32>,
    registers: [i64; 32],
    pub stack: Vec<i64>, // TODO: Find the size of the stack
    pub instructions: Vec<u32>,
}

impl Env {
    pub fn new() -> Self {
        // alias -> xN
        let register_alias = [
            ("zero", 0),
            ("ra", 1),
            ("sp", 2),
            ("gp", 3),
            ("tp", 4),
            ("t0", 5),
            ("t1", 6),
            ("t2", 7),
            ("s0", 8),
            ("s1", 9),
            ("a0", 10),
            ("a1", 11),
            ("a2", 12),
            ("a3", 13),
            ("a4", 14),
            ("a5", 15),
            ("a6", 16),
            ("a7", 17),
            ("s2", 18),
            ("s3", 19),
            ("s4", 20),
            ("s5", 21),
            ("s6", 22),
            ("s7", 23),
            ("s8", 24),
            ("s9", 25),
            ("s10", 26),
            ("s11", 27),
            ("t3", 28),
            ("t4", 29),
            ("t5", 30),
            ("t6", 31),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_owned()))
        .collect::<HashMap<_, _>>();

        Self {
            register_alias,
            labels: HashMap::new(),
            registers: [0; 32],
            stack: Vec::new(),
            instructions: Vec::new(),
        }
    }

    pub fn set_register(&mut self, reg: u32, value: i64) {
        self.registers[reg as usize] = value;
    }

    pub fn get_register(&self, reg: u32) -> i64 {
        self.registers[reg as usize]
    }

    pub fn alias_to_register(&self, reg: &str) -> Option<u32> {
        self.register_alias.get(reg).copied()
    }
    pub fn xn_to_register(&self, reg: &str) -> Option<u32> {
        if reg == "x0" {
            Some(0)
        } else if reg.starts_with("x") && !reg[1..].starts_with("0") {
            match reg[1..].parse::<u32>() {
                Ok(n) if n < 32 => Some(n),
                _ => None,
            }
        } else {
            None
        }
    }
    pub fn is_valid_register(&self, reg: &str) -> bool {
        self.alias_to_register(reg)
            .or_else(|| self.xn_to_register(reg))
            .is_some()
    }

    pub fn add_label(&mut self, label: &str, value: u32) {
        self.labels.insert(label.to_string(), value);
    }

    pub fn get_label(&self, label: &str) -> Option<u32> {
        self.labels.get(label).copied()
    }

    pub fn to_instruction(&self, tokens: Vec<(Token, Loc)>) -> Result<u32, RuntimeErr> {
        let (op, args) = match &tokens[0].0 {
            Token::Op(op, args) => (op, args),
            _ => unreachable!(),
        };

        todo!()
    }
}
