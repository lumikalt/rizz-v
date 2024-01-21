use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
        Config,
    },
};
use riscv_interpreter::{env::Env, parser::parse};

fn main() -> anyhow::Result<()> {
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = Config::default();
    let input = std::fs::read_to_string("test.s").unwrap();

    let file = SimpleFile::new("test.s", input.clone());

    let env = Env::new();

    match parse(&env, &input) {
        Ok(tokens) => {
            println!("{:#?} -> {:#?}", input, tokens);
        }
        Err(errs) => {
            for err in errs {
                let start = err.1.start;
                let end = err.1.end + 1;

                let diagnostic = Diagnostic::error()
                    .with_message("Syntax Error")
                    .with_labels(vec![
                        Label::primary((), start..end).with_message(err.0.to_string())
                    ])
                    .with_notes({
                        let mut notes = Vec::new();
                        if let Some(note) = err.3 {
                            notes.push(note);
                        }
                        notes.push(err.0.note());
                        notes
                    });

                term::emit(&mut writer.lock(), &config, &file, &diagnostic).unwrap();
            }
        }
    };

    Ok(())
}
