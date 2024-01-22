#[cfg(test)]
/// Test values come from Ripes
use crate::{
    env::Env,
    instructions::{get_instruction, with},
};

#[test]
#[ignore = "TODO"]
fn nop() {
    #[rustfmt::skip]
    {
        // nop (pseudo) -> addi x0, x0, 0
        // I-Type
        // |   imm12    |  ra |f3 |  rd | opcode
        //  000000000000 00000 000 00000 0010011
    };
    // nop
    assert_eq!(
        u32::from_str_radix(
            &with(
                get_instruction("nop"),
                0, // imm
                vec![]
            )
            .0
            .to_string(),
            2
        )
        .unwrap(),
        0b00000000000000000000000000010011u32
    );
}

#[test]
fn sb() {
    let env = Env::new();

    #[rustfmt::skip]
    {
        // S-Type
        // |  imm  |  rb |  ra |f3 |  rd | opcode
        //  1111111 11110 00010 000 11100 0100011
    };
    // sb t5 -4(sp)
    assert_eq!(
        u32::from_str_radix(
            &with(
                get_instruction("sb"),
                -4i32 as u32, // imm
                vec![
                    0,                                    // rd
                    env.alias_to_register("sp").unwrap(), // ra
                    env.alias_to_register("t5").unwrap()  // rb
                ],
            )
            .0
            .to_string(),
            2
        )
        .unwrap(),
        0b11111111111000010000111000100011u32
    );
}

#[test]
fn add() {
    let env = Env::new();

    #[rustfmt::skip]
    {
        // R-Type
        // |  f7   |  rb |  ra |f3 |  rd | opcode
        //  0000000 01011 01010 000 01010 0110011
    };
    // add a0 a0 a1
    assert_eq!(
        u32::from_str_radix(
            &with(
                get_instruction("add"),
                0, // imm
                vec![
                    env.alias_to_register("a0").unwrap(), // rd
                    env.alias_to_register("a0").unwrap(), // ra
                    env.alias_to_register("a1").unwrap()  // rb
                ]
            )
            .0
            .to_string(),
            2
        )
        .unwrap(),
        0b00000000101101010000010100110011u32
    );
}

#[test]
fn addi() {
    let env = Env::new();

    #[rustfmt::skip]
    {
        // I-Type
        // |   imm12    |  ra |f3 |  rd | opcode
        //  000000000001 01010 000 01010 0010011
        //  000000000001 01010 000 01010 0010011 (Ripes)
    };
    // addi a0 a0 1
    assert_eq!(
        u32::from_str_radix(
            with(
                get_instruction("addi"),
                1,
                vec![
                    env.alias_to_register("a0").unwrap(),
                    env.alias_to_register("a0").unwrap()
                ],
            )
            .0
            .to_string()
            .as_str(),
            2
        )
        .unwrap(),
        0b00000000000101010000010100010011u32
    );
}

#[test]
fn beq() {
    let env = Env::new();

    #[rustfmt::skip]
    {
        // B-Type
        // |  imm7 |  rb |  ra |f3 |imm5 | opcode
        //  0000000 01011 01010 000 00100 1100011
        //  0000000 01011 01010 000 00100 1100011 (Ripes)
    };
    // beq a0 a1 4
    assert_eq!(
        u32::from_str_radix(
            &with(
                get_instruction("beq"),
                4,
                vec![
                    0, // no rd
                    env.alias_to_register("a0").unwrap(),
                    env.alias_to_register("a1").unwrap()
                ]
            )
            .0
            .to_string(),
            2
        )
        .unwrap(),
        0b00000000101101010000001001100011u32
    );
}
