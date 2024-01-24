pub mod kind {
    use std::{
        fmt::{self, Display, Formatter},
        mem,
    };

    use bitfield::bitfield;

    /// will be converted by the engine to real instructions
    #[derive(Debug)]
    pub struct Pseudo(pub &'static str);

    bitfield! {
        pub struct R(u32);
        impl Debug;

        pub funct7, set_funct7: 31, 25;
        pub rb, set_rb: 24, 20;
        pub ra, set_ra: 19, 15;
        pub funct3, set_funct3: 14, 12;
        pub rd, set_rd: 11, 7;
        pub opcode, set_opcode: 6, 0;
    }

    bitfield! {
        pub struct R4(u32);
        impl Debug;

        pub rc, set_rc: 31, 27;
        pub funct2, set_funct2: 26, 25;
        pub rb, set_rb: 24, 20;
        pub ra, set_ra: 19, 15;
        pub funct3, set_funct3: 14, 12;
        pub rd, set_rd: 11, 7;
        pub opcode, set_opcode: 6, 0;
    }

    bitfield! {
        pub struct I(u32);
        impl Debug;

        pub imm, set_imm: 31, 20;
        pub ra, set_ra: 19, 15;
        pub funct3, set_funct3: 14, 12;
        pub rd, set_rd: 11, 7;
        pub opcode, set_opcode: 6, 0;
    }

    bitfield! {
        pub struct I2(u32);
        impl Debug;

        pub funct6, set_funct6: 31, 26;
        pub imm, set_imm: 25, 20;
        pub ra, set_ra: 19, 15;
        pub funct3, set_funct3: 14, 12;
        pub rd, set_rd: 11, 7;
        pub opcode, set_opcode: 6, 0;
    }

    bitfield! {
        pub struct S(u32);
        impl Debug;

        pub imm_11_5, set_imm_11_5: 31, 25;
        pub rb, set_rb: 24, 20;
        pub ra, set_ra: 19, 15;
        pub funct3, set_funct3: 14, 12;
        pub imm_4_0, set_imm_4_10: 11, 7;
        pub opcode, set_opcode: 6, 0;
    }

    bitfield! {
        pub struct B(u32);
        impl Debug;

        pub imm_12, set_imm_12: 31;
        pub imm_10_5, set_imm_10_5: 30, 25;
        pub rb, set_rb: 24, 20;
        pub ra, set_ra: 19, 15;
        pub funct3, set_funct3: 14, 12;
        pub imm_4_1, set_imm_4_1: 11, 8;
        pub imm_11, set_imm_11: 7;
        pub opcode, set_opcode: 6, 0;
    }

    bitfield! {
        pub struct U(u32);
        impl Debug;

        pub imm31_12, set_imm_31_12: 31, 12;
        pub rd, set_rd: 11, 7;
        pub opcode, set_opcode: 6, 0;
    }

    bitfield! {
        pub struct J(u32);
        impl Debug;

        pub imm_20, set_imm_20: 31;
        pub imm_10_1, set_imm_10_1: 30, 21;
        pub imm_11, set_imm_11: 20;
        pub imm_19_12, set_imm_19_12: 19, 12;
        pub rd, set_rd: 11, 7;
        pub opcode, set_opcode: 6, 0;
    }

    #[derive(Debug)]
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

    impl Kind {
        pub fn get_opcode(&self) -> Option<u32> {
            match self {
                Kind::Pseudo(_) => None,
                Kind::R(r) => Some(r.opcode()),
                Kind::R4(r4) => Some(r4.opcode()),
                Kind::I(i) => Some(i.opcode()),
                Kind::I2(i2) => Some(i2.opcode()),
                Kind::S(s) => Some(s.opcode()),
                Kind::B(b) => Some(b.opcode()),
                Kind::U(u) => Some(u.opcode()),
                Kind::J(j) => Some(j.opcode()),
            }
        }

        pub fn get_imm(&self) -> Option<u32> {
            match self {
                Kind::Pseudo(_) => None,
                Kind::R(_) => None,
                Kind::R4(_) => None,
                Kind::I(i) => Some(i.imm()),
                Kind::I2(i2) => Some(i2.imm() >> 20),
                Kind::S(s) => Some(s.imm_11_5() | s.imm_4_0()),
                Kind::B(b) => Some(
                    ((b.imm_12() as u32) << 12)
                        | ((b.imm_11() as u32) << 11)
                        | (b.imm_10_5() << 5)
                        | (b.imm_4_1() << 1),
                ),
                Kind::U(u) => Some(u.imm31_12() << 12),
                Kind::J(j) => Some(
                    ((j.imm_20() as u32) << 20)
                        | ((j.imm_19_12() as u32) << 12)
                        | ((j.imm_11() as u32) << 11)
                        | (j.imm_10_1() << 1),
                ),
            }
        }

        pub fn get_regs(&self) -> Option<Vec<usize>> {
            match self {
                Kind::Pseudo(_) => None,
                Kind::R(r) => Some(vec![r.rd() as usize, r.ra() as usize, r.rb() as usize, 0]),
                Kind::R4(r4) => Some(vec![
                    r4.rd() as usize,
                    r4.ra() as usize,
                    r4.rb() as usize,
                    r4.rc() as usize,
                ]),
                Kind::I(i) => Some(vec![i.rd() as usize, i.ra() as usize, 0, 0]),
                Kind::I2(i2) => Some(vec![i2.rd() as usize, i2.ra() as usize, 0, 0]),
                Kind::S(s) => Some(vec![0, s.ra() as usize, s.rb() as usize, 0]),
                Kind::B(b) => Some(vec![0, b.ra() as usize, b.rb() as usize, 0]),
                Kind::U(u) => Some(vec![u.rd() as usize, 0, 0, 0]),
                Kind::J(j) => Some(vec![j.rd() as usize, 0, 0, 0]),
            }
        }

        pub fn to_op(instruction: u32) -> (Kind, String) {
            let opcode = instruction & 0b00000000000000000000000001111111;

            match opcode {
                0b0110111 => (
                    Kind::U(unsafe { mem::transmute_copy(&instruction) }),
                    "lui".into(),
                ),
                0b0010011 => (
                    Kind::I(unsafe { mem::transmute_copy(&instruction) }),
                    "addi".into(),
                ),
                _ => todo!(),
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
                self.funct7(),
                self.rb(),
                self.ra(),
                self.funct3(),
                self.rd(),
                self.opcode()
            )
        }
    }
    impl Display for R4 {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{:05b}{:02b}{:05b}{:05b}{:03b}{:05b}{:07b}",
                self.rc(),
                self.funct2(),
                self.rb(),
                self.ra(),
                self.funct3(),
                self.rd(),
                self.opcode()
            )
        }
    }
    impl Display for I {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{:012b}{:05b}{:03b}{:05b}{:07b}",
                self.imm(),
                self.ra(),
                self.funct3(),
                self.rd(),
                self.opcode()
            )
        }
    }
    impl Display for I2 {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{:06b}{:06b}{:05b}{:03b}{:05b}{:07b}",
                self.funct6(),
                self.imm(),
                self.ra(),
                self.funct3(),
                self.rd(),
                self.opcode()
            )
        }
    }
    impl Display for S {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{:07b}{:05b}{:05b}{:03b}{:05b}{:07b}",
                self.imm_11_5(),
                self.rb(),
                self.ra(),
                self.funct3(),
                self.imm_4_0(),
                self.opcode()
            )
        }
    }
    impl Display for B {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}{:06b}{:05b}{:05b}{:03b}{:04b}{}{:07b}",
                self.imm_12() as u32,
                self.imm_10_5(),
                self.rb(),
                self.ra(),
                self.funct3(),
                self.imm_4_1(),
                self.imm_11() as u32,
                self.opcode()
            )
        }
    }
    impl Display for U {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{:020b}{:05b}{:07b}",
                self.imm31_12(),
                self.rd(),
                self.opcode()
            )
        }
    }
    impl Display for J {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}{:09b}{}{:08b}{:05b}{:07b}",
                self.imm_20() as u32,
                self.imm_10_1(),
                self.imm_11() as u32,
                self.imm_19_12(),
                self.rd(),
                self.opcode()
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
            Kind::U({
                let mut u = U(0);
                u.set_opcode(0b0110111);
                u
            }),
            vec![Arg::Register(0), Arg::Immediate],
        ),

        // Memory
        "sb" => (
            Kind::S({
                let mut s = S(0);
                s.set_funct3(0b000);
                s.set_opcode(0b0100011);
                s
            }),
            vec![Arg::Register(2), Arg::Memory],
        ),

        // Arithmetic, Logic, Shift
        "add" => (
            Kind::R({
                let mut r = R(0);
                r.set_funct7(0b0000000);
                r.set_funct3(0b000);
                r.set_opcode(0b0110011);
                r
            }),
            vec![Arg::Register(0), Arg::Register(1), Arg::Register(2)],
        ),
        "addi" => (
            Kind::I({
                let mut i = I(0);
                i.set_funct3(0b000);
                i.set_opcode(0b0010011);
                i
            }),
            vec![Arg::Register(0), Arg::Register(1), Arg::Immediate],
        ),

        // Multiply, Divide
        "mul" => (
            Kind::R({
                let mut r = R(0);
                r.set_funct7(0b0000001);
                r.set_funct3(0b000);
                r.set_opcode(0b0110011);
                r
            }),
            vec![Arg::Register(0), Arg::Register(1), Arg::Register(2)],
        ),
        "div" => (
            Kind::R({
                let mut r = R(0);
                r.set_funct7(0b0000001);
                r.set_funct3(0b100);
                r.set_opcode(0b0110011);
                r
            }),
            vec![Arg::Register(0), Arg::Register(1), Arg::Register(2)],
        ),

        // Compare

        // Flow control (branch, jump, call, ret)
        "beq" => (
            Kind::B({
                let mut b = B(0);
                b.set_funct3(0b000);
                b.set_opcode(0b1100011);
                b
            }),
            vec![Arg::Register(1), Arg::Register(2), Arg::Immediate],
        ),
        "bne" => (
            Kind::B({
                let mut b = B(0);
                b.set_funct3(0b001);
                b.set_opcode(0b1100011);
                b
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
            Kind::J({
                let mut j = J(0);
                j.set_opcode(0b1101111);
                j
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
pub fn with((kind, args): (Kind, Vec<Arg>), imm: u32, regs: Vec<usize>) -> (Kind, Vec<Arg>) {
    match kind {
        Kind::Pseudo(_) => (kind, args),
        Kind::R(mut r) => {
            r.set_rd(regs[0] as u32);
            r.set_ra(regs[1] as u32);
            r.set_rb(regs[2] as u32);
            (Kind::R(r), args)
        }
        Kind::R4(mut r4) => {
            r4.set_rd(regs[0] as u32);
            r4.set_ra(regs[1] as u32);
            r4.set_rb(regs[2] as u32);
            r4.set_rc(regs[3] as u32);
            (Kind::R4(r4), args)
        }
        Kind::I(mut i) => {
            i.set_rd(regs[0] as u32);
            i.set_ra(regs[1] as u32);
            i.set_imm(imm);
            (Kind::I(i), args)
        }
        Kind::I2(mut i2) => {
            i2.set_rd(regs[0] as u32);
            i2.set_ra(regs[1] as u32);
            i2.set_imm(imm);
            (Kind::I2(i2), args)
        }
        Kind::S(mut s) => {
            s.set_ra(regs[1] as u32);
            s.set_rb(regs[2] as u32);
            s.set_imm_11_5(imm >> 5);
            s.set_imm_4_10(imm);
            (Kind::S(s), args)
        }
        Kind::B(mut b) => {
            b.set_ra(regs[1] as u32);
            b.set_rb(regs[2] as u32);
            b.set_imm_12(to_bits::<1>(imm >> 12)[0]);
            b.set_imm_11(to_bits::<1>(imm >> 11)[0]);
            b.set_imm_10_5(imm >> 5);
            b.set_imm_4_1(imm >> 1);
            (Kind::B(b), args)
        }
        Kind::U(mut u) => {
            u.set_rd(regs[0] as u32);
            u.set_imm_31_12(imm >> 12);
            (Kind::U(u), args)
        }
        Kind::J(mut j) => {
            j.set_rd(regs[0] as u32);
            j.set_imm_20(to_bits::<1>(imm >> 20)[0]);
            j.set_imm_19_12(imm >> 12);
            j.set_imm_11(to_bits::<1>(imm >> 11)[0]);
            j.set_imm_10_1(imm >> 1);
            (Kind::J(j), args)
        }
    }
}

/// regs order: rd, ra, rb, rc
pub fn handle_pseudo(
    (kind, args): (Kind, Vec<Arg>),
    imm: u32,
    regs: Vec<usize>,
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
                    // addi rd, x0, imm
                    vec![with(get_instruction("addi"), imm, regs)]
                }
                // if the immediate is a multiple of 0x1000, use lui
                _ if imm & 0xfff == 0 => {
                    // lui rd, imm
                    vec![with(get_instruction("lui"), imm, regs)]
                }
                // otherwise, use lui and addi
                _ => vec![
                    // lui rd, imm
                    with(get_instruction("lui"), imm & 0xfffff000, regs.clone()),
                    // addi rd, rd, imm
                    with(
                        get_instruction("addi"),
                        imm & 0x00000fff,
                        vec![regs[0], regs[0]],
                    ),
                ],
            }
        }
        "beqz" => vec![
            // beq ra, x0, imm
            with(get_instruction("beq"), imm, regs),
        ],
        "bnez" => vec![
            // bne ra, x0, imm
            with(get_instruction("bne"), imm, regs),
        ],
        "j" => vec![
            // jal x0, imm
            with(get_instruction("jal"), imm, regs),
        ],
        other => {
            dbg!(other);
            unimplemented!()
        }
    }
}

const fn to_bits<const N: usize>(val: u32) -> [bool; N] {
    let mut bits = [false; N];
    for i in 0..N {
        bits[i] = (val >> i) & 1 == 1;
    }
    bits
}
