use std::collections::HashMap;

#[derive(Debug)]
pub struct Env {
    pub register_alias: HashMap<String, usize>,
    labels: HashMap<String, usize>,
    registers: [i64; 32],
    pub stack: Vec<i64>, // TODO: Find the size of the stack
}

impl Env {
    pub fn new() -> Self {
        // alias -> xN
        let register_alias = [
            ("zero", 0),
            ("ra", 1),
            ("sp", 2),
            ("gp", 3),
            ("tp", 4),
            ("t0", 5),
            ("t1", 6),
            ("t2", 7),
            ("s0", 8),
            ("s1", 9),
            ("a0", 10),
            ("a1", 11),
            ("a2", 12),
            ("a3", 13),
            ("a4", 14),
            ("a5", 15),
            ("a6", 16),
            ("a7", 17),
            ("s2", 18),
            ("s3", 19),
            ("s4", 20),
            ("s5", 21),
            ("s6", 22),
            ("s7", 23),
            ("s8", 24),
            ("s9", 25),
            ("s10", 26),
            ("s11", 27),
            ("t3", 28),
            ("t4", 29),
            ("t5", 30),
            ("t6", 31),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_owned()))
        .collect::<HashMap<_, _>>();

        Self {
            register_alias,
            labels: HashMap::new(),
            registers: [0; 32],
            stack: Vec::new(),
        }
    }

    pub fn set_register(&mut self, reg: usize, value: i64) {
        self.registers[reg] = value;
    }

    pub fn get_register(&self, reg: usize) -> i64 {
        self.registers[reg]
    }

    pub fn alias_to_register(&self, reg: &str) -> Option<usize> {
        self.register_alias.get(reg).copied()
    }
    pub fn xn_to_register(&self, reg: &str) -> Option<usize> {
        if reg == "x0" {
            Some(0)
        } else if reg.starts_with("x") && !reg[1..].starts_with("0") {
            match reg[1..].parse::<usize>() {
                Ok(n) if n < 32 => Some(n),
                _ => None,
            }
        } else {
            None
        }
    }
    pub fn is_valid_register(&self, reg: &str) -> bool {
        self.alias_to_register(reg).or_else(|| self.xn_to_register(reg)).is_some()
    }

    pub fn add_label(&mut self, label: &str, value: usize) {
        self.labels.insert(label.to_string(), value);
    }

    pub fn get_label(&self, label: &str) -> Option<usize> {
        self.labels.get(label).copied()
    }
}
