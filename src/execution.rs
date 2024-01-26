use crate::{env::Env, instructions::kind::Kind};

/// lui rd, imm
fn lui(env: &mut Env, rd: usize, imm: u32) {
    env.set_register(rd, imm);
}

/// add rd, ra, rb
fn add(env: &mut Env, rd: usize, ra: usize, rb: usize) {
    env.set_register(rd, env.get_register(ra).wrapping_add(env.get_register(rb)));
}

/// addi rd, ra, imm
fn addi(env: &mut Env, rd: usize, ra: usize, imm: u32) {
    env.set_register(rd, env.get_register(ra).wrapping_add(imm));
}

/// xor rd, ra, rb
fn xor(env: &mut Env, rd: usize, ra: usize, rb: usize) {
    env.set_register(rd, env.get_register(ra) ^ env.get_register(rb));
}

/// mul rd, ra, rb
fn mul(env: &mut Env, rd: usize, ra: usize, rb: usize) {
    env.set_register(rd, env.get_register(ra).wrapping_mul(env.get_register(rb)));
}

/// beq ra, rb, imm
fn beq(env: &mut Env, ra: usize, rb: usize, imm: u32) -> bool {
    if env.get_register(ra) == env.get_register(rb) {
        env.pc = env.pc.wrapping_add(imm);
        return true;
    }
    false
}

/// jal rd, imm
fn jal(env: &mut Env, rd: usize, imm: u32) {
    env.set_register(rd, env.pc + 4);
    env.pc = env.pc.wrapping_add(imm);
}

/// fmadd.s fd, fa, fb, fc
fn fmadd_s(env: &mut Env, fd: usize, fa: usize, fb: usize, fc: usize) {
    env.set_fregister(
        fd,
        env.get_fregister(fa) * env.get_fregister(fb) + env.get_fregister(fc),
    );
}

pub fn run_instruction(env: &mut Env, instruction: u32) -> bool {
    let (kind, _) = Kind::to_op(instruction);
    let mut regs = kind.get_regs().unwrap();
    // Ensure all four registers have a value
    regs.extend([0].repeat(4 - regs.len()));
    let (rd, ra, rb) = (regs[0], regs[1], regs[2]);
    let (fd, fa, fb, fc) = (regs[0], regs[1], regs[2], regs[3]);
    let imm = kind.get_imm();
    let opcode = kind.get_opcode().unwrap();
    let funct3 = instruction >> 12 & 0b111;
    let funct7 = instruction >> 25 & 0b1111111;

    match opcode {
        0b0110111 => {
            lui(env, rd, imm.unwrap());
            false
        }
        0b0110011 if funct3 == 0b000 && funct7 == 0b0000000 => {
            add(env, rd, ra, rb);
            false
        }
        0b0010011 => {
            addi(env, rd, ra, imm.unwrap());
            false
        }
        0b0110011 if funct3 == 0b100 && funct7 == 0b0000000 => {
            xor(env, rd, ra, rb);
            false
        }
        0b0110011 if funct3 == 0b000 && funct7 == 0b0000001 => {
            mul(env, rd, ra, rb);
            false
        }
        0b1100011 => beq(env, ra, rb, imm.unwrap()),
        0b1101111 => {
            jal(env, rd, imm.unwrap());
            true
        }
        0b1000011 => {
            fmadd_s(env, fd, fa, fb, fc);
            false
        }
        _ => todo!(),
    }
}
