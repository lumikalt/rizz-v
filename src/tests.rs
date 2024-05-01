#[cfg(test)]
/// Test values come from Ripes
use crate::{
    env::Env,
    instructions::{get_instruction, handle_pseudo, with},
};

#[test]
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
        handle_pseudo(
            get_instruction("nop"),
            0, // imm
            vec![]
        )[0]
        .0
        .to_u32(),
        0b00000000000000000000000000010011
    );
}

#[test]
fn li() {
    let env = Env::new();

    #[rustfmt::skip]
    {
        // li (pseudo) -> Myriad Sequence (in this case, both addi and lui)
        // 1 -> lui
        // U-Type
        // |       imm20        |  rd | opcode
        //  00000000000000001101 01010 0110111
        // 2 -> addi
        // I-Type
        // |   imm12    |  ra |f3 |  rd | opcode
        //  000000101001 01010 000 01010 0010011
    };
    // li a0 53289
    assert!(handle_pseudo(
        get_instruction("li"),
        53289,
        vec![env.str_to_register("a0").unwrap()]
    )
    .into_iter()
    .map(|i| i.0.to_u32())
    .eq([
        0b00000000000000001101010100110111,
        0b00000010100101010000010100010011
    ]
    .into_iter()));
}

#[test]
fn lui() {
    let env = Env::new();

    #[rustfmt::skip]
    {
        // U-Type
        // |       imm20        |  rd | opcode
        //  00000011010100101001 01010 0110111
    };
    // lui a0 13609
    assert_eq!(
        with(
            get_instruction("lui"),
            13609 << 12,
            vec![env.str_to_register("a0").unwrap()]
        )
        .0
        .to_u32(),
        0b00000011010100101001010100110111
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
        with(
            get_instruction("sb"),
            -4i32 as u32, // imm
            vec![
                0,                                  // rd
                env.str_to_register("sp").unwrap(), // ra
                env.str_to_register("t5").unwrap()  // rb
            ],
        )
        .0
        .to_u32(),
        0b11111111111000010000111000100011
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
        with(
            get_instruction("add"),
            0, // imm
            vec![
                env.str_to_register("a0").unwrap(), // rd
                env.str_to_register("a0").unwrap(), // ra
                env.str_to_register("a1").unwrap()  // rb
            ]
        )
        .0
        .to_u32(),
        0b00000000101101010000010100110011
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
    };
    // addi a0 a0 1
    assert_eq!(
        with(
            get_instruction("addi"),
            1,
            vec![
                env.str_to_register("a0").unwrap(),
                env.str_to_register("a0").unwrap()
            ],
        )
        .0
        .to_u32(),
        0b00000000000101010000010100010011
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
    };
    // beq a0 a1 4
    assert_eq!(
        with(
            get_instruction("beq"),
            4,
            vec![
                0, // no rd
                env.str_to_register("a0").unwrap(),
                env.str_to_register("a1").unwrap()
            ]
        )
        .0
        .to_u32(),
        0b00000000101101010000001001100011
    );
}
