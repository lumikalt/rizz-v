use std::collections::HashMap;

use crate::{
    err::RuntimeErr,
    instructions::{handle_pseudo, instruction, kind::Kind, with, Arg},
    parser::{Loc, Token},
};

pub enum Variables {
    Byte(u8),
    Half(u16),
    Word(u32),
    DWord(u64),
    String(String),
}

#[derive(Debug)]
pub struct Env {
    register_alias: HashMap<String, u32>,
    labels: HashMap<String, u32>,
    registers: [u32; 32],
    /// EQ, LT, GT
    pub cmp_flags: [bool; 3],
    pub stack: Vec<u32>, // TODO: Find the actual size of the stack
    pub instructions: Vec<u32>,
    pub pc: u32,
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
            cmp_flags: [false; 3],
            stack: Vec::from([0; 1024]), // 1024 * 64 = 64 KiB stack
            instructions: Vec::new(),
            pc: 0,
        }
    }

    pub fn set_register(&mut self, reg: u32, value: u32) {
        self.registers[reg as usize] = value;
    }

    pub fn get_register(&self, reg: u32) -> u32 {
        self.registers[reg as usize]
    }

    pub fn str_to_register(&self, reg: &str) -> Option<u32> {
        if reg == "x0" {
            Some(0)
        } else if reg.starts_with("x") && !reg[1..].starts_with("0") {
            match reg[1..].parse::<u32>() {
                Ok(n) if n < 32 => Some(n),
                _ => None,
            }
        } else {
            self.register_alias.get(reg).copied()
        }
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
    ) -> Result<Vec<u32>, (RuntimeErr, Loc, Option<String>)> {
        if let (Token::Op(name, args), loc) = op {
            let i = if let Some(i) = instruction(&name) {
                i
            } else {
                return Err((
                    RuntimeErr::InvalidOp,
                    loc,
                    Some("no implementation exists".to_string()),
                ));
            };
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
                                    RuntimeErr::InvalidType(
                                        Arg::from(args[k].0.clone()).kind(),
                                        v.kind(),
                                    ),
                                    args[k].1,
                                    None,
                                ))
                            }
                        }
                        Arg::Register(id) => {
                            if let Token::Register(r) = &args[k].0 {
                                regs[id] = self.str_to_register(&r).unwrap();
                                Ok(())
                            } else {
                                Err((
                                    RuntimeErr::InvalidType(
                                        Arg::from(args[k].0.clone()).kind(),
                                        v.kind(),
                                    ),
                                    args[k].1,
                                    None,
                                ))
                            }
                        }
                        Arg::Memory => {
                            if let Token::Memory(i, r) = &args[k].0 {
                                if r.is_some() {
                                    regs[k] = self
                                        .str_to_register(&if let Token::Register(r) =
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
                                    RuntimeErr::InvalidType(
                                        Arg::from(args[k].0.clone()).kind(),
                                        v.kind(),
                                    ),
                                    args[k].1,
                                    None,
                                ))
                            }
                        }
                        Arg::Symbol => {
                            if let Token::Symbol(s) = &args[k].0 {
                                if let Some(v) = self.get_label(&s) {
                                    imm = (v).wrapping_sub(loc.mem_offset as u32);
                                    Ok(())
                                } else {
                                    Err((
                                        RuntimeErr::InvalidType(
                                            Arg::from(args[k].0.clone()).kind(),
                                            v.kind(),
                                        ),
                                        args[k].1,
                                        None,
                                    ))
                                }
                            } else {
                                Err((
                                    RuntimeErr::InvalidType(
                                        Arg::from(args[k].0.clone()).kind(),
                                        v.kind(),
                                    ),
                                    args[k].1,
                                    None,
                                ))
                            }
                        }
                    })?;
            Ok({
                if let Kind::Pseudo(_) = i.0 {
                    handle_pseudo(i, imm, regs)
                        .into_iter()
                        .map(|x| u32::from_str_radix(&x.0.to_string(), 2).unwrap())
                        .collect()
                } else {
                    vec![u32::from_str_radix(&with(i, imm, regs).0.to_string(), 2).unwrap()]
                }
            })
        } else {
            unreachable!()
        }
    }

    pub fn handle_mem_offsets(&mut self, mut tokens: Vec<(Token, Loc)>) -> Vec<(Token, Loc)> {
        let mut i = 0;
        // Calculate the instruction position for all opcodes to
        // allow for labels to be used before they are defined
        tokens
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(id, (token, _))| match token {
                Token::Op(name, _) => {
                    if let Some((kind, args)) = instruction(&name) {
                        if let Kind::Pseudo(_) = kind {
                            tokens[id].1.mem_offset = i + 4;
                            handle_pseudo((kind, args), 0, vec![0; 4])
                                .into_iter()
                                .for_each(|_| i += 4);
                        } else {
                            i += 4;
                        }
                    }
                }
                Token::Label(name) => {
                    self.add_label(&name, (i + 4) as u32);
                }
                other => {
                    dbg!(other);
                    unreachable!()
                }
            });

        tokens
    }

    /// Assume memory offsets have been handled
    pub fn exec_op(&mut self, (op, loc): (Token, Loc)) -> Result<(), (RuntimeErr, Loc, Option<String>)> {
        let (i, args) = if let Token::Op(name, args) = op {
            if let Some(i) = instruction(&name) {
                (i, args.clone())
            } else {
                return Err((
                    RuntimeErr::InvalidOp,
                    loc,
                    Some("no implementation exists".to_string()),
                ));
            }
        } else {
            unreachable!()
        };

        todo!()
    }
}
