use std::io::Write;

use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
        Config,
    },
};
use colored::Colorize;
use itertools::Itertools;
use rizz_v::{
    env::Env,
    execution::run_instruction,
    info::info,
    parser::{parse, Token},
};
use termion::input::TermRead;

fn main() -> anyhow::Result<()> {
    let display_mode = 's';
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = Config::default();
    let input = std::fs::read_to_string("test.s").unwrap();
    let term_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);

    let file = SimpleFile::new("test.s", input.clone());

    let mut env = Env::new();

    let mut toks: Vec<Token> = Vec::new();
    let mut ops: Vec<u32> = Vec::new();

    let mut parse_asm_result = String::new();

    match parse(&env, &input) {
        Ok(tokens) => {
            let lines: Vec<&str> = input.lines().collect();
            let size = lines.iter().map(|l| l.len()).max().unwrap();

            env.handle_mem_offsets(tokens)
                .iter()
                .for_each(|(token, loc)| {
                    let token = token.clone();

                    match token.clone() {
                        Token::Mnemonic(..) => {
                            match env.assemble_op((token.clone(), loc.clone())) {
                                Ok(op) => {
                                    let mut formatted = format!(
                                        "{:<1$} {3:02x}: {2:032b}",
                                        lines[loc.line - 1],
                                        size + 3,
                                        op[0],
                                        loc.mem_offset
                                    );
                                    ops.push(op[0]);
                                    toks.push(token.clone());

                                    if op.len() > 1 {
                                        for op in op[1..].iter() {
                                            formatted += &format!(
                                                "\n{:<1$} {3:02x}: {2:032b}",
                                                "",
                                                size + 3,
                                                op,
                                                loc.mem_offset
                                            );
                                            ops.push(*op);
                                            toks.push(token.clone());
                                        }
                                    }
                                    parse_asm_result += &format!("{}\n", formatted);
                                }
                                Err(err) => {
                                    let diagnostic = Diagnostic::error()
                                        .with_message("Engine Error")
                                        .with_labels(vec![Label::primary(
                                            (),
                                            err.1.start..(err.1.end + 1),
                                        )
                                        .with_message(err.0.to_string())])
                                        .with_notes({
                                            let mut notes = Vec::new();
                                            if let Some(note) = &err.2 {
                                                notes.push(note.to_string());
                                            }
                                            notes.push(err.0.note());
                                            notes
                                        });

                                    term::emit(&mut writer.lock(), &config, &file, &diagnostic)
                                        .unwrap();
                                }
                            }
                        }
                        Token::Label(name) => {
                            parse_asm_result += &format!(
                                "{:<1$}     <{2:02x}>\n",
                                name.clone() + ":",
                                size + 3,
                                env.get_label(&name).unwrap()
                            );
                        }
                        _ => unreachable!(),
                    }
                });
        }
        Err(errs) => {
            let err = errs.first().unwrap();
            let start = err.1.start;
            let end = err.1.end + 1;

            let diagnostic = Diagnostic::error()
                .with_message("Syntax Error")
                .with_labels(vec![
                    Label::primary((), start..end).with_message(err.0.to_string())
                ])
                .with_notes({
                    let mut notes = Vec::new();
                    if let Some(note) = &err.3 {
                        notes.push(note.to_string());
                    }
                    notes.push(err.0.note());
                    notes
                });

            term::emit(&mut writer.lock(), &config, &file, &diagnostic).unwrap();

            return Ok(());
        }
    };

    let mut file = std::fs::File::create("test.bin")?;
    for op in ops.iter() {
        let formatted = format!("{:08x}\n", op);
        file.write_all(formatted.as_bytes()).unwrap();
    }

    // Print the register values

    while env.pc / 4 < ops.clone().len() as u32 {
        let pc = env.pc.clone();
        let prev_regs = env.registers.clone();
        let prev_fregs = env.fregisters.clone();

        env.pc += 4 * !run_instruction(&mut env, ops[pc as usize >> 2]) as u32;

        let mut changed = Vec::new();
        for (i, _) in prev_regs
            .iter()
            .zip(env.registers.iter())
            .enumerate()
            .filter(|(_, (prev, curr))| prev != curr)
        {
            changed.push(i);
        }

        let mut fchanged = Vec::new();
        for (i, _) in prev_fregs
            .iter()
            .zip(env.fregisters.iter())
            .enumerate()
            .filter(|(_, (prev, curr))| prev != curr)
        {
            fchanged.push(i);
        }

        println!(
            "{}\n",
            parse_asm_result
                .lines()
                .enumerate()
                .map(|(i, line)| {
                    if i == pc as usize >> 2 {
                        format!("> {}", line).bright_green()
                    } else {
                        format!("  {}", line).normal()
                    }
                })
                .join("\n")
        );
        let (right, tag) = if let Token::Mnemonic(op, args) = &toks[pc as usize >> 2] {
            info(
                &env,
                &op,
                args.iter().map(|(token, _)| token.to_string()).collect(),
                display_mode,
            )
        } else {
            unreachable!()
        };
        let left = make_box(
            term_width as u32 / 2,
            pc as usize,
            env.registers.clone().into_iter().collect(),
            changed,
            display_mode,
            true,
            tag.clone(),
        ) + &make_box_fp(
            term_width as u32 / 2,
            env.fregisters.clone().into_iter().collect(),
            fchanged,
            display_mode,
            true,
            tag,
        );

        println!(
            "{}",
            left.lines()
                .zip(right.lines().chain([""].repeat(left.lines().count())))
                .map(|(l, r)| format!("{}   {}", l, r))
                .join("\n")
        );

        println!("\nPress enter to continue...");
        for c in std::io::stdin().keys() {
            match c.unwrap() {
                termion::event::Key::Char('\n') => break,
                _ => {}
            }
        }
    }

    Ok(())
}

const fn round_down_to_power_of_two(n: u32) -> u32 {
    1 << (32 - n.leading_zeros() - 1)
}

/// Assuming the terminal is at least 80 characters wide
///
/// Display Mode:
/// - s: signed decimal
/// - u: unsigned decimal
/// - b: binary
/// - h: hex
fn make_box(
    width: u32,
    pc: usize,
    regs: Vec<u32>,
    changed: Vec<usize>,
    display_mode: char,
    first: bool,
    tag: (Vec<usize>, Vec<usize>),
) -> String {
    let cell_inner_width: u32 = match display_mode {
        'b' => 32,
        'u' => 10,
        'h' => 8,
        's' => 11,
        _ => unreachable!(),
    } + 7;

    // Nnumber of boxes that fit horizontally
    let num_boxes = round_down_to_power_of_two((width / (cell_inner_width + 2)) as u32);
    if num_boxes <= 1 {
        return make_one_wide_box(pc, regs, changed, display_mode, first, tag);
    }
    let mut boxed = String::new();

    if first {
        boxed += &format!(
            "┌─╢ pc = {pc:04x} ╟{:─<1$}┬",
            "",
            cell_inner_width.saturating_sub(14) as usize
        );
        for _ in 1..(num_boxes - 1) {
            boxed += &format!("{:─<1$}┬", "", cell_inner_width as usize);
        }
        boxed += &format!("{:─<1$}┐\n", "", cell_inner_width as usize);
    } else {
        boxed += &format!(
            "├─╢ pc = {pc:04x} ╟{:─<1$}┼",
            "",
            cell_inner_width.saturating_sub(14) as usize
        );
        for _ in 1..(num_boxes - 1) {
            boxed += &format!("{:─<1$}┼", "", cell_inner_width as usize);
        }
        boxed += &format!("{:─<1$}┤\n", "", cell_inner_width as usize);
    }

    for chunk in &regs.iter().enumerate().chunks(num_boxes as usize) {
        let chunk = chunk.collect::<Vec<_>>();
        let mut formatted = String::from("│ ");

        for (i, reg) in chunk {
            let reg = match display_mode {
                'b' => format!("x{:<3} {1:0>32b}", i.to_string() + ":", reg),
                'u' => format!("x{:<3} {1:>10}", i.to_string() + ":", reg),
                'h' => format!("x{:<3} {1:0>8x}", i.to_string() + ":", reg),
                's' => {
                    let signed = *reg as i32;
                    let sign = if signed < 0 { "-" } else { "+" };
                    format!(
                        "x{:<3} {:>11}",
                        i.to_string() + ":",
                        sign.to_string() + &signed.abs().to_string()
                    )
                }
                _ => unreachable!(),
            };
            let reg = if changed.contains(&i) {
                reg.bright_green()
            } else {
                if tag.0.contains(&i) {
                    reg.bright_yellow()
                } else {
                    reg.normal()
                }
            };
            formatted += &format!("{} │ ", reg);
        }

        boxed += &format!("{}\n", formatted);
    }

    boxed
}

fn make_one_wide_box(
    pc: usize,
    regs: Vec<u32>,
    changed: Vec<usize>,
    display_mode: char,
    first: bool,
    tag: (Vec<usize>, Vec<usize>),
) -> String {
    let mut boxed = String::new();

    boxed += &if first {
        format!("┌─╢ pc = {pc:04x} ╟{:─<1$}┐\n", "", 32)
    } else {
        format!("├─╢ pc = {pc:04x} ╟{:─<1$}┤\n", "", 32)
    };
    for (i, reg) in regs.iter().enumerate() {
        let reg = match display_mode {
            'b' => format!("x{:<3} {1:0>32b}", i.to_string() + ":", reg),
            'u' => format!("x{:<3} {1:0>10}", i.to_string() + ":", reg),
            'h' => format!("x{:<3} {1:0>8x}", i.to_string() + ":", reg),
            's' => {
                let signed = *reg as i32;
                let sign = if signed < 0 { "-" } else { "+" };
                format!("x{:<3} {}{:0>10}", i.to_string() + ":", sign, signed.abs())
            }
            _ => unreachable!(),
        };
        let reg = if changed.contains(&i) {
            reg.bright_green()
        } else {
            if tag.0.contains(&i) {
                reg.bright_yellow()
            } else {
                reg.normal()
            }
        };
        boxed += &format!("│ {} │\n", reg);
    }

    boxed
}

/// Assuming the terminal is at least 80 characters wide
/// Display Mode:
/// - s,u: floating point
/// - b: binary
/// - x: hexadecimal
/// - e: scientific
fn make_box_fp(
    width: u32,
    regs: Vec<f32>,
    changed: Vec<usize>,
    display_mode: char,
    last: bool,
    tag: (Vec<usize>, Vec<usize>),
) -> String {
    let cell_inner_width: u32 = 11 + 7;

    // Nnumber of boxes that fit horizontally
    let num_boxes = round_down_to_power_of_two((width / (cell_inner_width + 2)) as u32);
    if num_boxes <= 1 {
        return make_one_wide_box_fp(regs, changed, last, tag);
    }
    let mut boxed = String::new();

    boxed += &format!("├─{:─<1$}┼", "", (cell_inner_width - 1) as usize);
    for _ in 1..(num_boxes - 1) {
        boxed += &format!("{:─<1$}┼", "", cell_inner_width as usize);
    }
    boxed += &format!("{:─<1$}┤\n", "", cell_inner_width as usize);

    for chunk in &regs.iter().enumerate().chunks(num_boxes as usize) {
        let chunk = chunk.collect::<Vec<_>>();
        let mut formatted = String::from("│ ");

        for (i, freg) in chunk {
            // let freg = format!("f{:<3} {:>11}", i.to_string() + ":", freg);
            let freg = match display_mode {
                's' | 'u' => format!("f{:<3} {:>11}", i.to_string() + ":", freg),
                'b' => format!("f{:<3} {:0>32b}", i.to_string() + ":", freg.to_bits()),
                'x' => format!("f{:<3} {:0>8x}", i.to_string() + ":", freg.to_bits()),
                'e' => format!("f{:<3} {:>11e}", i.to_string() + ":", freg),
                _ => unreachable!(),
            };
            let reg = if changed.contains(&i) {
                freg.bright_green()
            } else {
                if tag.1.contains(&i) {
                    freg.bright_yellow()
                } else {
                    freg.normal()
                }
            };
            formatted += &format!("{} │ ", reg);
        }

        boxed += &format!("{}\n", formatted);
    }

    if last {
        boxed += &format!("└{:─<1$}┴", "", cell_inner_width as usize);
        for _ in 1..(num_boxes - 1) {
            boxed += &format!("{:─<1$}┴", "", cell_inner_width as usize);
        }
        boxed += &format!("{:─<1$}┘", "", cell_inner_width as usize);
    } else {
        boxed += &format!("├{:─<1$}┼", "", cell_inner_width as usize);
        for _ in 1..(num_boxes - 1) {
            boxed += &format!("{:─<1$}┼", "", cell_inner_width as usize);
        }
        boxed += &format!("{:─<1$}┤", "", cell_inner_width as usize);
    }

    boxed
}

fn make_one_wide_box_fp(
    regs: Vec<f32>,
    changed: Vec<usize>,
    last: bool,
    tag: (Vec<usize>, Vec<usize>),
) -> String {
    let mut boxed = String::new();

    boxed += &format!("├─{:─<1$}┤\n", "", 32);

    for (i, freg) in regs.iter().enumerate() {
        let freg = format!("f{:<3} {:0>11}", i.to_string() + ":", freg);
        let reg = if changed.contains(&i) {
            freg.bright_green()
        } else {
            if tag.1.contains(&i) {
                freg.bright_yellow()
            } else {
                freg.normal()
            }
        };
        boxed += &format!("│ {} │\n", reg);
    }
    boxed += &if last {
        format!("└{:─<1$}┘", "", 32)
    } else {
        format!("├{:─<1$}┤", "", 32)
    };

    boxed
}
