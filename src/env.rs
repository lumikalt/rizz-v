use std::collections::HashMap;

#[derive(Debug)]
pub struct Env {
    register_alias: HashMap<String, usize>,
    labels: HashMap<String, usize>,
    registers: [i64; 32],
    stack: Vec<i64>, // TODO: Find the size of the stack
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

    pub fn add_label(&mut self, label: &str, value: usize) {
        self.labels.insert(label.to_string(), value);
    }

    pub fn get_label(&self, label: &str) -> Option<usize> {
        self.labels.get(label).copied()
    }
}
