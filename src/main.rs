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
    env::Env,
    parser::{parse, Token},
};

fn main() -> anyhow::Result<()> {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = Config::default();
    let input = std::fs::read_to_string("test.s").unwrap();

    let file = SimpleFile::new("test.s", input.clone());

    let mut env = Env::new();

    match parse(&env, &input) {
        Ok(tokens) => {
            let lines: Vec<&str> = input.lines().collect();
            let size = lines.iter().map(|l| l.len()).max().unwrap();

            tokens.iter().enumerate().for_each(|(line, token)| {
                let token = token.clone();
                env.handle_labels(tokens.clone());

                match token.0 {
                    Token::Op(..) => match env.assemble_op(token) {
                        Ok(op) => {
                            let mut formatted = format!(
                                "{:<1$} {2:032b}",
                                lines[line].to_string() + ":",
                                size + 3,
                                op[0]
                            );

                            if op.len() > 1 {
                                for op in op[1..].iter() {
                                    formatted += &format!("{:<1$} {2:032b}", "", size + 3, op);
                                }
                            }
                            println!("{}", formatted);
                        }
                        Err(err) => {
                            let diagnostic = Diagnostic::error()
                                .with_message("Runtime Error")
                                .with_labels(vec![Label::primary((), err.1.start..(err.1.end + 1))
                                    .with_message(err.0.to_string())])
                                .with_notes({
                                    let mut notes = Vec::new();
                                    if let Some(note) = &err.2 {
                                        notes.push(note.to_string());
                                    }
                                    notes.push(err.0.note());
                                    notes
                                });

                            term::emit(&mut writer.lock(), &config, &file, &diagnostic).unwrap();
                        }
                    },
                    Token::Label(name) => {
                        println!("{name}:");
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

    Ok(())
}
