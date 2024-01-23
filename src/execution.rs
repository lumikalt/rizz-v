use crate::{env::Env, instructions::kind::Kind};

/// lui rd, imm
fn lui(env: &mut Env, rd: usize, imm: u32) {
    env.set_register(rd, imm);
}

/// jal rd, imm
fn jal(env: &mut Env, rd: usize, imm: u32) {
    env.set_register(rd, env.pc);
    env.pc += imm;
}

const fn to_u32<const N: usize>(bits: &[bool; N]) -> u32 {
    let mut val = 0;
    for i in 0..N {
        if bits[i] {
            val |= 1 << i;
        }
    }
    val
}

pub fn run_instruction(env: &mut Env, kind: Kind) {
    let mut regs = kind.get_regs().unwrap();
    // Ensure all four registers have a value
    regs.extend([0].repeat(4 - regs.len()));
    let (rd, ra, rb, _rc) = (regs[0], regs[1], regs[2], regs[3]);
    let imm = kind.get_imm().unwrap();
    let opcode = kind.get_opcode().unwrap();

    match to_u32(opcode) {
        0b0110111 => {
            lui(env, rd, imm)
        }
        _ => todo!(),
    }
}
