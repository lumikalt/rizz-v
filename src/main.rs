use rayon::prelude::*;
use riscv_interpreter::parser::{parse, Token};

fn main() {
    let input = std::fs::read_to_string("test.s").unwrap();
    let tokens = parse(&input);
    // println!("{:#?} -> {:#?}", input, tokens.into_par_iter().filter(|(token, _)| !matches!(token, Token::Spacing)).collect::<Vec<_>>());
    println!("{:#?} -> {:#?}", input, tokens);
}
