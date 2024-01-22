use std::collections::HashMap;

use crate::{
    err::RuntimeErr,
    instructions::{instruction, with, Arg},
    parser::{Loc, Token},
};

pub enum SymbolValue {
    Byte(u8),
    Half(u16),
    Word(u32),
    DWord(u64),
    String(String),
}

#[derive(Debug)]
pub struct Env {
    pub register_alias: HashMap<String, u32>,
    labels: HashMap<String, u32>,
    registers: [u64; 32],
    pub stack: Vec<u64>, // TODO: Find the size of the stack
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
            ("fp", 8),
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
            stack: Vec::from([0; 1024]), // 1024 * 64 = 64 KiB stack
            instructions: Vec::new(),
        }
    }

    pub fn set_register(&mut self, reg: u32, value: u64) {
        self.registers[reg as usize] = value;
    }

    pub fn get_register(&self, reg: u32) -> u64 {
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
    pub fn reg_to_register(&self, reg: &str) -> Option<u32> {
        if reg.starts_with("x") {
            self.xn_to_register(reg)
        } else {
            self.alias_to_register(reg)
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

    pub fn assemble_op(
        &self,
        op: (Token, Loc),
    ) -> Result<u32, (RuntimeErr, Loc, Option<String>)> {
        if let (Token::Op(name, args), loc) = op {
            let i = instruction(&name);
            let mut imm = 0u32;
            let mut regs = vec![0; 4];
            if args.len() != i.1.len() {
                return Err((
                    RuntimeErr::InvalidOpArity(name, args.len(), i.1.len()),
                    loc,
                    None,
                ));
            }

            let _ =
                i.1.clone()
                    .into_iter()
                    .enumerate()
                    .try_for_each(|(k, v)| match v {
                        Arg::Immediate => {
                            if let Token::Immediate(i) = args[k].0 {
                                imm = i as u32;
                                Ok(())
                            } else {
                                Err((
                                    RuntimeErr::InvalidType("immediate".to_string(), v.kind()),
                                    args[k].1,
                                    None,
                                ))
                            }
                        }
                        Arg::Register(id) => {
                            if let Token::Register(r) = &args[k].0 {
                                regs[id] = self.reg_to_register(&r).unwrap();
                                Ok(())
                            } else {
                                Err((
                                    RuntimeErr::InvalidType("register".to_string(), v.kind()),
                                    args[k].1,
                                    None,
                                ))
                            }
                        }
                        Arg::Memory => {
                            if let Token::Memory(i, r) = &args[k].0 {
                                if r.is_some() {
                                    regs[k] = self
                                        .reg_to_register(&if let Token::Register(r) =
                                            *(r.clone().unwrap())
                                        {
                                            r
                                        } else {
                                            unreachable!()
                                        })
                                        .unwrap();
                                }
                                imm = if let Token::Immediate(i) = **i {
                                    i as u32
                                } else {
                                    unreachable!()
                                };
                                Ok(())
                            } else {
                                Err((
                                    RuntimeErr::InvalidType("memory".to_string(), v.kind()),
                                    args[k].1,
                                    None,
                                ))
                            }
                        }
                        _ => unimplemented!(),
                    })?;
            Ok(u32::from_str_radix(dbg!(&with(i, imm, regs).0.to_string()), 2).unwrap())
        } else {
            unreachable!()
        }
    }
}
