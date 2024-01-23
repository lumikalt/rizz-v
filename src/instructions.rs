pub mod kind {
    use std::fmt::{self, Display, Formatter};

    use crate::instructions::to_u32;

    /// will be converted by the engine to a real instruction
    pub struct Pseudo(pub &'static str);

    pub struct R {
        pub funct7: [bool; 7],
        pub rb: [bool; 5],
        pub ra: [bool; 5],
        pub funct3: [bool; 3],
        pub rd: [bool; 5],
        pub opcode: [bool; 7],
    }

    pub struct R4 {
        pub rc: [bool; 5],
        pub funct2: [bool; 2],
        pub rb: [bool; 5],
        pub ra: [bool; 5],
        pub funct3: [bool; 3],
        pub rd: [bool; 5],
        pub opcode: [bool; 7],
    }

    pub struct I {
        /// 11:0
        pub imm: [bool; 12],
        pub ra: [bool; 5],
        pub funct3: [bool; 3],
        pub rd: [bool; 5],
        pub opcode: [bool; 7],
    }

    pub struct I2 {
        pub funct6: [bool; 6],
        /// 5:0
        pub imm: [bool; 6],
        pub ra: [bool; 5],
        pub funct3: [bool; 3],
        pub rd: [bool; 5],
        pub opcode: [bool; 7],
    }

    pub struct S {
        /// 11:5
        pub imm: [bool; 7],
        pub rb: [bool; 5],
        pub ra: [bool; 5],
        pub funct3: [bool; 3],
        /// 4:0
        pub imm2: [bool; 5],
        pub opcode: [bool; 7],
    }

    pub struct B {
        /// 12 | 10:5
        pub imm: [bool; 7],
        pub rb: [bool; 5],
        pub ra: [bool; 5],
        pub funct3: [bool; 3],
        /// 4:1 | 11
        pub imm2: [bool; 5],
        pub opcode: [bool; 7],
    }

    pub struct U {
        /// 31:12
        pub imm: [bool; 20],
        pub rd: [bool; 5],
        pub opcode: [bool; 7],
    }

    pub struct J {
        /// 20 | 10:1 | 11 | 19:12
        pub imm: [bool; 20],
        pub rd: [bool; 5],
        pub opcode: [bool; 7],
    }

    pub enum Kind {
        Pseudo(Pseudo),
        R(R),
        R4(R4),
        I(I),
        I2(I2),
        S(S),
        B(B),
        U(U),
        J(J),
    }

    impl Display for Kind {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Kind::Pseudo(pseudo) => write!(f, "{}", pseudo),
                Kind::R(r) => write!(f, "{}", r),
                Kind::R4(r4) => write!(f, "{}", r4),
                Kind::I(i) => write!(f, "{}", i),
                Kind::I2(i2) => write!(f, "{}", i2),
                Kind::S(s) => write!(f, "{}", s),
                Kind::B(b) => write!(f, "{}", b),
                Kind::U(u) => write!(f, "{}", u),
                Kind::J(j) => write!(f, "{}", j),
            }
        }
    }

    impl Display for Pseudo {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{:0^32}", 0)
        }
    }
    impl Display for R {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{:07b}{:05b}{:05b}{:03b}{:05b}{:07b}",
                to_u32(&self.funct7),
                to_u32(&self.rb),
                to_u32(&self.ra),
                to_u32(&self.funct3),
                to_u32(&self.rd),
                to_u32(&self.opcode),
            )
        }
    }
    impl Display for R4 {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{:05b}{:02b}{:05b}{:05b}{:03b}{:05b}{:07b}",
                to_u32(&self.rc),
                to_u32(&self.funct2),
                to_u32(&self.rb),
                to_u32(&self.ra),
                to_u32(&self.funct3),
                to_u32(&self.rd),
                to_u32(&self.opcode),
            )
        }
    }
    impl Display for I {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{:012b}{:05b}{:03b}{:05b}{:07b}",
                to_u32(&self.imm),
                to_u32(&self.ra),
                to_u32(&self.funct3),
                to_u32(&self.rd),
                to_u32(&self.opcode),
            )
        }
    }
    impl Display for I2 {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{:06b}{:06b}{:05b}{:03b}{:05b}{:07b}",
                to_u32(&self.funct6),
                to_u32(&self.imm),
                to_u32(&self.ra),
                to_u32(&self.funct3),
                to_u32(&self.rd),
                to_u32(&self.opcode),
            )
        }
    }
    impl Display for S {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{:07b}{:05b}{:05b}{:03b}{:05b}{:07b}",
                to_u32(&self.imm),
                to_u32(&self.rb),
                to_u32(&self.ra),
                to_u32(&self.funct3),
                to_u32(&self.imm2),
                to_u32(&self.opcode),
            )
        }
    }
    impl Display for B {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{:07b}{:05b}{:05b}{:03b}{:05b}{:07b}",
                to_u32(&self.imm),
                to_u32(&self.rb),
                to_u32(&self.ra),
                to_u32(&self.funct3),
                to_u32(&self.imm2),
                to_u32(&self.opcode),
            )
        }
    }
    impl Display for U {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{:020b}{:05b}{:07b}",
                to_u32(&self.imm),
                to_u32(&self.rd),
                to_u32(&self.opcode),
            )
        }
    }
    impl Display for J {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{:020b}{:05b}{:07b}",
                to_u32(&self.imm),
                to_u32(&self.rd),
                to_u32(&self.opcode),
            )
        }
    }
}

#[derive(Debug, Clone)]
pub enum Arg {
    /// rd -> 0, ra -> 1, rb -> 2, rc -> 3
    Register(usize),
    Immediate,
    /// always ra
    Memory,
    // It's just an immediate but different name in the ref sheet
    Symbol,
}

impl Arg {
    pub fn kind(&self) -> String {
        match self {
            Arg::Register(_) => "register",
            Arg::Immediate => "immediate",
            Arg::Memory => "memory",
            Arg::Symbol => "symbol",
        }
        .to_string()
    }
}

impl From<Token> for Arg {
    fn from(token: Token) -> Self {
        match token {
            Token::Immediate(_) => Arg::Immediate,
            Token::Register(_) => Arg::Register(0),
            Token::Memory(_, _) => Arg::Memory,
            Token::Symbol(_) => Arg::Symbol,
            _ => unreachable!(),
        }
    }
}

use kind::*;

use crate::parser::Token;

/// (kind, (arity, Vec<token kind>))
pub fn instruction(op: &str) -> Option<(Kind, Vec<Arg>)> {
    Some(match op {
        // -
        "nop" => (Kind::Pseudo(Pseudo("nop")), vec![]),

        // Move
        "li" => (
            Kind::Pseudo(Pseudo("li")),
            vec![Arg::Register(0), Arg::Immediate],
        ),
        "lui" => (
            Kind::U(U {
                imm: to_bits(0),
                rd: to_bits(0),
                opcode: to_bits(0b0110111),
            }),
            vec![Arg::Register(0), Arg::Immediate],
        ),

        // Memory
        "sb" => (
            Kind::S(S {
                imm: to_bits(0),
                rb: to_bits(0),
                ra: to_bits(0),
                funct3: to_bits(0b000),
                imm2: to_bits(0),
                opcode: to_bits(0b0100011),
            }),
            vec![Arg::Register(2), Arg::Memory],
        ),

        // Arithmetic, Logic, Shift
        "add" => (
            Kind::R(R {
                funct7: to_bits(0b0000000),
                rb: to_bits(0),
                ra: to_bits(0),
                funct3: to_bits(0b000),
                rd: to_bits(0),
                opcode: to_bits(0b0110011),
            }),
            vec![Arg::Register(0), Arg::Register(1), Arg::Register(2)],
        ),
        "addi" => (
            Kind::I(I {
                imm: to_bits(0),
                ra: to_bits(0),
                funct3: to_bits(0b000),
                rd: to_bits(0),
                opcode: to_bits(0b0010011),
            }),
            vec![Arg::Register(0), Arg::Register(1), Arg::Immediate],
        ),

        // Multiply, Divide
        "mul" => (
            Kind::R(R {
                funct7: to_bits(0b0000001),
                rb: to_bits(0),
                ra: to_bits(0),
                funct3: to_bits(0b000),
                rd: to_bits(0),
                opcode: to_bits(0b0110011),
            }),
            vec![Arg::Register(0), Arg::Register(1), Arg::Register(2)],
        ),
        "div" => (
            Kind::R(R {
                funct7: to_bits(0b0000001),
                rb: to_bits(0),
                ra: to_bits(0),
                funct3: to_bits(0b100),
                rd: to_bits(0),
                opcode: to_bits(0b0110011),
            }),
            vec![Arg::Register(0), Arg::Register(1), Arg::Register(2)],
        ),

        // Compare

        // Flow control (branch, jump, call, ret)
        "beq" => (
            Kind::B(B {
                imm: to_bits(0),
                rb: to_bits(0),
                ra: to_bits(0),
                funct3: to_bits(0b000),
                imm2: to_bits(0),
                opcode: to_bits(0b1100011),
            }),
            vec![Arg::Register(1), Arg::Register(2), Arg::Immediate],
        ),
        "bne" => (
            Kind::B(B {
                imm: to_bits(0),
                rb: to_bits(0),
                ra: to_bits(0),
                funct3: to_bits(0b001),
                imm2: to_bits(0),
                opcode: to_bits(0b1100011),
            }),
            vec![Arg::Register(1), Arg::Register(2), Arg::Immediate],
        ),
        "beqz" => (
            Kind::Pseudo(Pseudo("beqz")),
            vec![Arg::Register(1), Arg::Symbol],
        ),
        "bnez" => (
            Kind::Pseudo(Pseudo("bnez")),
            vec![Arg::Register(1), Arg::Symbol],
        ),
        "j" => (Kind::Pseudo(Pseudo("j")), vec![Arg::Symbol]),
        "jal" => (
            Kind::J(J {
                imm: to_bits(0),
                rd: to_bits(0),
                opcode: to_bits(0b1101111),
            }),
            vec![Arg::Register(0), Arg::Symbol],
        ),
        op => unimplemented!("{}", op),
    })
}

pub fn get_instruction(op: &str) -> (Kind, Vec<Arg>) {
    unsafe { instruction(op).unwrap_unchecked() }
}

/// regs order: rd, ra, rb, rc
pub fn with((kind, args): (Kind, Vec<Arg>), imm: u32, regs: Vec<u32>) -> (Kind, Vec<Arg>) {
    match kind {
        Kind::Pseudo(_) => (kind, args),
        Kind::R(r) => (
            Kind::R(R {
                funct7: r.funct7,
                rb: to_bits(regs[2]),
                ra: to_bits(regs[1]),
                funct3: r.funct3,
                rd: to_bits(regs[0]),
                opcode: r.opcode,
            }),
            args,
        ),
        Kind::R4(r4) => (
            Kind::R4(R4 {
                rc: to_bits(regs[3]),
                funct2: r4.funct2,
                rb: to_bits(regs[2]),
                ra: to_bits(regs[1]),
                funct3: r4.funct3,
                rd: to_bits(regs[0]),
                opcode: r4.opcode,
            }),
            args,
        ),
        Kind::I(i) => (
            Kind::I(I {
                imm: to_bits(imm),
                ra: to_bits(regs[1]),
                funct3: i.funct3,
                rd: to_bits(regs[0]),
                opcode: i.opcode,
            }),
            args,
        ),
        Kind::I2(i2) => (
            Kind::I2(I2 {
                funct6: i2.funct6,
                imm: to_bits(imm),
                ra: to_bits(regs[1]),
                funct3: i2.funct3,
                rd: to_bits(regs[0]),
                opcode: i2.opcode,
            }),
            args,
        ),
        Kind::S(s) => {
            let bits = to_bits::<32>(imm);
            (
                Kind::S(S {
                    imm: {
                        let mut imm = [false; 7];
                        imm.copy_from_slice(&bits[5..=11]); //.into_iter().rev().map(|&b| b).collect::<Vec<_>>()[..]);
                        imm
                    },
                    rb: to_bits(regs[2]),
                    ra: to_bits(regs[1]),
                    funct3: s.funct3,
                    imm2: {
                        let mut imm2 = [false; 5];
                        imm2.copy_from_slice(&bits[0..=4]); //.into_iter().rev().map(|&b| b).collect::<Vec<_>>()[..]);
                        imm2
                    },
                    opcode: s.opcode,
                }),
                args,
            )
        }
        Kind::B(b) => {
            let bits = to_bits::<32>(imm);
            (
                Kind::B(B {
                    imm: {
                        let mut imm = [false; 7];
                        imm[6] = bits[12];
                        imm[0..=5].copy_from_slice(&bits[5..=10]);
                        imm
                    },
                    rb: to_bits(regs[2]),
                    ra: to_bits(regs[1]),
                    funct3: b.funct3,
                    imm2: {
                        let mut imm2 = [false; 5];
                        imm2[1..=4].copy_from_slice(&bits[1..=4]);
                        imm2[0] = bits[11];
                        imm2
                    },
                    opcode: b.opcode,
                }),
                args,
            )
        }
        Kind::U(u) => (
            Kind::U(U {
                imm: to_bits(imm >> 12), // 31:12
                rd: to_bits(regs[0]),
                opcode: u.opcode,
            }),
            args,
        ),
        Kind::J(j) => (
            Kind::J(J {
                imm: {
                    let bits = to_bits::<32>(imm);
                    let mut imm = [false; 20];
                    imm[19] = bits[20];
                    imm[9..=18].copy_from_slice(&bits[1..=10]);
                    imm[8] = bits[11];
                    imm[0..=7].copy_from_slice(&bits[12..=19]);
                    imm
                },
                rd: to_bits(regs[0]),
                opcode: j.opcode,
            }),
            args,
        ),
    }
}

/// regs order: rd, ra, rb, rc
pub fn handle_pseudo(
    (kind, args): (Kind, Vec<Arg>),
    imm: u32,
    regs: Vec<u32>,
) -> Vec<(Kind, Vec<Arg>)> {
    let op = if let Kind::Pseudo(Pseudo(op)) = kind {
        op
    } else {
        return vec![(kind, args)];
    };

    match op {
        "nop" => vec![
            // addi x0, x0, 0
            with(get_instruction("addi"), 0, vec![0, 0]),
        ],
        "li" => {
            match imm {
                // if the immediate is small enough (12 bits), use addi
                _ if imm >> 12 == 0 => {
                    vec![with(get_instruction("addi"), imm, regs)]
                }
                // if the immediate is a multiple of 0x1000, use lui
                _ if imm & 0xfff == 0 => {
                    vec![with(get_instruction("lui"), imm, regs)]
                }
                // otherwise, use lui and addi
                _ => vec![
                    with(get_instruction("lui"), imm & 0xfffff000, regs.clone()),
                    with(get_instruction("addi"), imm & 0x00000fff, regs),
                ],
            }
        }
        "beqz" => vec![
            // beq ra, x0, imm
            with(get_instruction("beq"), imm, vec![0, regs[0], 0]),
        ],
        "bnez" => vec![
            // bne ra, x0, imm
            with(get_instruction("bne"), imm, vec![0, regs[0], 0]),
        ],
        "j" => vec![
            // jal x0, imm
            with(get_instruction("jal"), imm, vec![0]),
        ],
        other => {
            dbg!(other);
            unimplemented!()
        }
    }
}

fn to_bits<const N: usize>(val: u32) -> [bool; N] {
    let mut bits = [false; N];
    for i in 0..N {
        bits[i] = (val >> i) & 1 == 1;
    }
    bits
}

fn to_u32<const N: usize>(bits: &[bool; N]) -> u32 {
    let mut val = 0;
    for i in 0..N {
        if bits[i] {
            val |= 1 << i;
        }
    }
    val
}
