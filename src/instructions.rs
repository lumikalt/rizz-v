pub mod kind {
    use std::fmt::{self, Display, Formatter};

    use crate::instructions::to_u32;

    /// will be converted by the engine to a real instruction
    pub struct Pseudo {}

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
            // (pseudo) padded on either side with - to make it 32 characters
            write!(f, "{:-^32}", "(pseudo)")
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

pub enum Value {
    Register,
    Immediate,
    Memory,
    Symbol,
}

use kind::*;

/// (kind, (arity, Vec<token kind>))
pub fn instruction(op: &str) -> (Kind, Vec<Value>) {
    match op {
        // -
        "nop" => (Kind::Pseudo(Pseudo {}), vec![]),

        // Move
        "li" => (
            Kind::Pseudo(Pseudo {}),
            vec![Value::Register, Value::Immediate],
        ),

        // Memory

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
            vec![Value::Register, Value::Register, Value::Register],
        ),
        "addi" => (
            Kind::I(I {
                imm: to_bits(0),
                ra: to_bits(0),
                funct3: to_bits(0b000),
                rd: to_bits(0),
                opcode: to_bits(0b0010011),
            }),
            vec![Value::Register, Value::Register, Value::Immediate],
        ),
        _ => unimplemented!(),
    }
}

/// Order: rd, ra, rb, rc
pub fn with_reg_args((kind, args): (Kind, Vec<Value>), regs: Vec<u32>) -> (Kind, Vec<Value>) {
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
                imm: i.imm,
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
                imm: i2.imm,
                ra: to_bits(regs[1]),
                funct3: i2.funct3,
                rd: to_bits(regs[0]),
                opcode: i2.opcode,
            }),
            args,
        ),
        Kind::S(s) => (
            Kind::S(S {
                imm: s.imm,
                rb: to_bits(regs[2]),
                ra: to_bits(regs[1]),
                funct3: s.funct3,
                imm2: s.imm2,
                opcode: s.opcode,
            }),
            args,
        ),
        Kind::B(b) => (
            Kind::B(B {
                imm: b.imm,
                rb: to_bits(regs[2]),
                ra: to_bits(regs[1]),
                funct3: b.funct3,
                imm2: b.imm2,
                opcode: b.opcode,
            }),
            args,
        ),
        Kind::U(u) => (
            Kind::U(U {
                imm: u.imm,
                rd: to_bits(regs[0]),
                opcode: u.opcode,
            }),
            args,
        ),
        Kind::J(j) => (
            Kind::J(J {
                imm: j.imm,
                rd: to_bits(regs[0]),
                opcode: j.opcode,
            }),
            args,
        ),
    }
}

pub fn with_imm((kind, args): (Kind, Vec<Value>), imm: u32) -> (Kind, Vec<Value>) {
    match kind {
        Kind::Pseudo(_) => (kind, args),
        Kind::R(r) => (Kind::R(r), args),
        Kind::R4(r4) => (Kind::R4(r4), args),
        Kind::I(i) => (
            Kind::I(I {
                imm: to_bits(imm),
                ra: i.ra,
                funct3: i.funct3,
                rd: i.rd,
                opcode: i.opcode,
            }),
            args,
        ),
        Kind::I2(i2) => (
            Kind::I2(I2 {
                funct6: i2.funct6,
                imm: to_bits(imm),
                ra: i2.ra,
                funct3: i2.funct3,
                rd: i2.rd,
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
                        imm.copy_from_slice(&bits[11..=5]);
                        imm
                    },
                    rb: s.rb,
                    ra: s.ra,
                    funct3: s.funct3,
                    imm2: {
                        let mut imm2 = [false; 5];
                        imm2.copy_from_slice(&bits[4..=0]);
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
                        imm[5..=0].copy_from_slice(&bits[10..=5]);
                        imm
                    },
                    rb: b.rb,
                    ra: b.ra,
                    funct3: b.funct3,
                    imm2: {
                        let mut imm2 = [false; 5];
                        imm2[4..=1].copy_from_slice(&bits[4..=1]);
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
                imm: {
                    let bits = to_bits::<32>(imm);
                    let mut imm = [false; 20];
                    imm.copy_from_slice(&bits[31..=12]);
                    imm
                },
                rd: u.rd,
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
                    imm[18..=9].copy_from_slice(&bits[10..=1]);
                    imm[8] = bits[11];
                    imm[7..=0].copy_from_slice(&bits[19..=12]);
                    imm
                },
                rd: j.rd,
                opcode: j.opcode,
            }),
            args,
        ),
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
