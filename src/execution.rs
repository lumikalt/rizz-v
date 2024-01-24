use crate::{env::Env, instructions::kind::Kind};

/// lui rd, imm
fn lui(env: &mut Env, rd: usize, imm: u32) {
    env.set_register(rd, imm);
}

/// addi rd, ra, imm
fn addi(env: &mut Env, rd: usize, ra: usize, imm: u32) {
    env.set_register(rd, env.get_register(ra) + imm);
}

/// xor rd, ra, rb
fn xor(env: &mut Env, rd: usize, ra: usize, rb: usize) {
    env.set_register(rd, env.get_register(ra) ^ env.get_register(rb));
}

/// jal rd, imm
fn jal(env: &mut Env, rd: usize, imm: u32) {
    env.set_register(rd, env.pc);
    env.pc += imm;
}

/// fmadd.s fd, fa, fb, fc
fn fmadd_s(env: &mut Env, fd: usize, fa: usize, fb: usize, fc: usize) {
    env.set_fregister(fd, env.get_fregister(fa) * env.get_fregister(fb) + env.get_fregister(fc));
}

pub fn run_instruction(env: &mut Env, kind: Kind) {
    let mut regs = kind.get_regs().unwrap();
    // Ensure all four registers have a value
    regs.extend([0].repeat(4 - regs.len()));
    let (rd, ra, rb) = (regs[0], regs[1], regs[2]);
    let (fd, fa, fb, fc) = (regs[0], regs[1], regs[2], regs[3]);
    let imm = kind.get_imm().unwrap();
    let opcode = kind.get_opcode().unwrap();

    match opcode {
        0b0110111 => {
            lui(env, rd, imm)
        }
        0b0010011 => {
            addi(env, rd, ra, imm)
        }
        0b0110011 => {
            xor(env, rd, ra, rb)
        }
        0b1101111 => {
            jal(env, rd, imm)
        }
        0b1000011 => {
            fmadd_s(env, fd, fa, fb, fc)
        }
        _ => todo!(),
    }
}
