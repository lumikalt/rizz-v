use std::collections::HashMap;

use colored::Colorize;

fn add_color(input: String) -> String {
    let map = [
        ("number", "magenta"),
        ("green", "blue"),
    ].iter().cloned().collect::<HashMap<&str, &str>>();

    let mut colored = String::new();

    for word in input.split(' ') {
        if let Some(num) = word.parse::<i32>().ok() {
            colored.push_str(&format!("{:032b}", num));
        }
    }

    colored
}

fn handle_color(input: &str, color: &str) -> String {
    match color {
        "magenta" => input.magenta().to_string(),
        "blue" => input.blue().to_string(),
        _ => input.to_string(),
    }
}
