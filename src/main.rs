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
use riscv_interpreter::{
    env::Env,
    execution::run_instruction,
    parser::{parse, Token},
};

fn main() -> anyhow::Result<()> {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = Config::default();
    let input = std::fs::read_to_string("test.s").unwrap();
    let term_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);

    let file = SimpleFile::new("test.s", input.clone());

    let mut env = Env::new();

    let mut ops: Vec<u32> = Vec::new();

    match parse(&env, &input) {
        Ok(tokens) => {
            let lines: Vec<&str> = input.lines().collect();
            let size = lines.iter().map(|l| l.len()).max().unwrap();

            env.handle_mem_offsets(tokens)
                .iter()
                .for_each(|(token, loc)| {
                    let token = token.clone();

                    match token.clone() {
                        Token::Op(..) => match env.assemble_op((token, loc.clone())) {
                            Ok(op) => {
                                let mut formatted = format!(
                                    "{:<1$} {3:02x}: {2:032b}",
                                    lines[loc.line - 1],
                                    size + 3,
                                    op[0],
                                    loc.mem_offset
                                );
                                ops.push(op[0]);

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
                                    }
                                }
                                println!("{}", formatted);
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
                        },
                        Token::Label(name) => {
                            println!(
                                "{:<1$}     <{2:02x}>",
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

    // Print the register values

    while env.pc / 4 < ops.clone().len() as u32 {
        let pc = env.pc.clone();
        let prev_regs = env.registers.clone();

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

        println!(
            "{}",
            make_box(
                term_width as u32,
                pc as usize,
                env.registers.clone().into_iter().collect(),
                changed,
                's'
            )
        );
    }

    Ok(())
}


fn round_down_to_power_of_two(n: u32) -> u32 {
    1 << (32 - n.leading_zeros() - 1)
}

/// Assuming the terminal is at least 80 characters wide
/// Display Mode:
/// - b: binary
/// - u: unsigned decimal
/// - h: hex
/// - s: signed decimal
/// - f: float
fn make_box(
    width: u32,
    pc: usize,
    regs: Vec<u32>,
    changed: Vec<usize>,
    display_mode: char,
) -> String {
    let cell_inner_width: u32 = match display_mode {
        'b' => 32,
        'u' => 10,
        'h' => 8,
        's' => 11,
        'f' => todo!("float display mode"),
        _ => unreachable!(),
    } + 7;

    // Nnumber of boxes that fit horizontally
    let num_boxes = round_down_to_power_of_two((width / (cell_inner_width + 2)) as u32);
    let mut boxed = String::new();

    boxed += &format!(
        "┌─╢ pc = {pc:04x} ╟{:─<1$}┬",
        "",
        cell_inner_width.saturating_sub(14) as usize
    );
    for _ in 1..(num_boxes - 1) {
        boxed += &format!("{:─<1$}┬", "", cell_inner_width as usize);
    }
    boxed += &format!("{:─<1$}┐\n", "", cell_inner_width as usize);

    for chunk in &regs.iter().enumerate().chunks(num_boxes as usize) {
        let chunk = chunk.collect::<Vec<_>>();
        let mut formatted = String::from("│ ");

        for (i, reg) in chunk {
            let reg = match display_mode {
                'b' => format!("x{:<3} {1:0>32b}", i.to_string() + ":", reg),
                'u' => format!("x{:<3} {1:0>10}", i.to_string() + ":", reg),
                'h' => format!("x{:<3} {1:0>8x}", i.to_string() + ":", reg),
                's' => {
                    let signed = *reg as i32;
                    let sign = if signed < 0 { "-" } else { "+" };
                    format!("x{:<3} {}{:0>10}", i.to_string() + ":", sign, signed)
                }
                'f' => todo!("float display mode"),
                _ => unreachable!(),
            };
            let reg = if changed.contains(&i) {
                reg.bright_green()
            } else {
                reg.normal()
            };
            formatted += &format!("{} │ ", reg);
        }

        boxed += &format!("{}\n", formatted);
    }

    boxed += &format!("└{:─<1$}┴", "", cell_inner_width as usize);
    for _ in 1..(num_boxes - 1) {
        boxed += &format!("{:─<1$}┴", "", cell_inner_width as usize);
    }
    boxed += &format!("{:─<1$}┘", "", cell_inner_width as usize);

    boxed
}
