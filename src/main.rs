use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
        Config,
    },
};
use riscv_interpreter::{
    env::Env, execution::run_instruction, info::info, instructions::{instruction, kind::Kind}, parser::{parse, Token}
};

fn main() -> anyhow::Result<()> {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = Config::default();
    let input = std::fs::read_to_string("test.s").unwrap();

    let file = SimpleFile::new("test.s", input.clone());

    let mut env = Env::new();

    let mut ops: Vec<(Kind, String)> = Vec::new();

    match parse(&env, &input) {
        Ok(tokens) => {
            let lines: Vec<&str> = input.lines().collect();
            let size = lines.iter().map(|l| l.len()).max().unwrap();

            env.handle_mem_offsets(tokens)
                .iter()
                .for_each(|(token, loc)| {
                    let token = token.clone();

                    match token.clone() {
                        Token::Op(name, _) => match env.assemble_op((token, loc.clone())) {
                            Ok(op) => {
                                let mut formatted = format!(
                                    "{:<1$} {3:02x}: {2:032b}",
                                    lines[loc.line - 1],
                                    size + 3,
                                    op[0],
                                    loc.mem_offset
                                );
                                ops.push(Kind::to_op(op[0].clone()));

                                if op.len() > 1 {
                                    for op in op[1..].iter() {
                                        formatted += &format!(
                                            "\n{:<1$} {3:02x}: {2:032b}",
                                            "",
                                            size + 3,
                                            op,
                                            loc.mem_offset
                                        );
                                        ops.push(Kind::to_op(op.clone()));
                                    }
                                }
                                println!("{}", formatted);
                            }
                            Err(err) => {
                                let diagnostic = Diagnostic::error()
                                    .with_message("Runtime Error")
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

    for op in ops {
        run_instruction(&mut env, op.0);

        println!(
            "{:<1$}     <{2:02x}>",
            op.1,
            32,
            env.get_register(10));
    }

    Ok(())
}
