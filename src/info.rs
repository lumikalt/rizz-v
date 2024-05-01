use colored::Colorize;

use crate::env::Env;

/// Display a helpful message about an instruction.
///
/// Returns the message and, if needed, tags to highlight specific registers ([x regs...], [fregs...]).
pub fn info(
    env: &Env,
    op: &str,
    args: Vec<String>,
    display_mode: char,
) -> (String, (Vec<usize>, Vec<usize>)) {
    let args: Vec<_> = args
        .into_iter()
        .map(|a| {
            if let Ok(num) = a.parse::<u32>() {
                match display_mode {
                    'd' => num.to_string(),
                    's' => (num as i32).to_string(),
                    'b' => format!("{:032b}", num),
                    'h' => format!("{:08x}", num),
                    _ => unreachable!(),
                }
            } else {
                a
            }
        })
        .collect();

    let mut tag = (vec![], vec![]);
    let msg = match op {
        "nop" => vec!["do nothing".to_string()],
        "li" => vec![format!(
            "load {} into {}",
            args[1].italic().yellow(),
            args[0].red()
        )],
        "lui" => {
            let imm = format!("{:032b}", args[1].parse::<i32>().unwrap() as u32)
                .chars()
                .rev()
                .collect::<String>();
            vec![
                format!("load the upper 20 bits of {} into {}", args[1], args[0]),
                format!(
                    "{} = {}{}",
                    args[1].italic().yellow(),
                    imm[12..32].to_string().green(),
                    imm[0..12].to_string().strikethrough().black()
                ),
                format!(
                    "{:>1$} ← {2} << 12",
                    args[0].blue(),
                    args[1].to_string().len(),
                    imm[12..32].to_string(),
                ),
            ]
        }
        "add" => {
            tag = (
                vec![
                    env.str_to_register(&args[1]).unwrap(),
                    env.str_to_register(&args[2]).unwrap(),
                ],
                vec![],
            );
            vec![format!(
                "add the values of {0} and {1} and store the result in {2}\n{2} ← {0} + {1}",
                args[1].blue(),
                args[2].blue(),
                args[0].blue()
            )]
        }
        "addi" => {
            tag = (
                vec![
                    env.str_to_register(&args[1]).unwrap(),
                    env.str_to_register(&args[2]).unwrap(),
                ],
                vec![],
            );
            vec![format!(
                "add the values of {0} and {1} and store the result in {2}\n{2} ← {0} + {1}",
                args[2].blue(),
                args[1].italic().yellow(),
                args[0].blue()
            )]
        }
        "sub" => vec![format!(
            "subtract the value of {} from the value of {} and store the result in {}",
            args[1].blue(),
            args[2].blue(),
            args[0].blue()
        )],
        "mul" => {
            tag = (
                vec![],
                vec![
                    env.str_to_fregister(&args[1]).unwrap(),
                    env.str_to_fregister(&args[2]).unwrap(),
                ],
            );
            vec![format!(
                "multiply the values of {0} and {1} and store the result in {2}\n{2} ← {0} ✕ {1}",
                args[1].blue(),
                args[2].blue(),
                args[0].blue()
            )]
        }
        "fadd.s" => {
            tag = (
                vec![],
                vec![
                    env.str_to_fregister(&args[1]).unwrap(),
                    env.str_to_fregister(&args[2]).unwrap(),
                ],
            );
            vec![format!(
                "add the values of {0} and {1} and store the result in {2}\n{2} ← {0} + {1}",
                args[1].blue(),
                args[2].blue(),
                args[0].blue()
            )]
        }
        "fdiv.s" => {
            tag = (
                vec![],
                vec![
                    env.str_to_fregister(&args[1]).unwrap(),
                    env.str_to_fregister(&args[2]).unwrap(),
                ],
            );
            vec![format!(
                "divide the value of {0} by the value of {1} and store the result in {2}\n{2} ← {0} ÷ {1}",
                args[1].blue(),
                args[2].blue(),
                args[0].blue()
            )]
        }
        "fcvt.s.w" => {
            tag = (vec![env.str_to_register(&args[1]).unwrap()], vec![]);
            vec![format!(
                "convert the value of {} to a float and store it in {}",
                args[1].blue(),
                args[0].blue()
            )]
        }
        "fmv.w.x" => {
            tag = (vec![env.str_to_register(&args[1]).unwrap()], vec![]);
            vec![format!(
                "interpret the value of {0} as a float and store it in {1}\n{1} ← {0}",
                args[1].blue(),
                args[0].blue()
            )]
        }
        op => todo!("{}", op),
    }
    .join("\n");

    (msg, tag)
}
