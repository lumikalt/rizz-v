use colored::Colorize;

use crate::env::Env;

pub fn info(env: &Env, op: &str, args: Vec<String>) -> Vec<String> {
    match op {
        "nop" => vec!["Do nothing - wait 1 cycle".to_string()],
        "li" => vec![format!("load {} into the register {}", args[1], args[0])],
        "lui" => {
            let imm = format!("{:032b}", args[1].parse::<i32>().unwrap() as u32)
                .chars()
                .rev()
                .collect::<String>();
            vec![
                format!(
                    "load the upper 20 bits of {} into the register {}",
                    args[1], args[0]
                ),
                format!(
                    "{} = {}{}",
                    args[1],
                    imm[12..32].to_string().red(),
                    imm[0..12].to_string()
                ),
                format!("{:>1$} = {2}{3:0>12}", args[0], args[1].to_string().len(), imm[12..32].to_string(), "0".to_string().bold()),
            ]
        }
        _ => todo!(),
    }
}
