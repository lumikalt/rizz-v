use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum SyntaxErr {
    TraillingComma,
    /// false for '(' true for ')'
    UnmatchedParen(bool),
    UnexpectedChar,
    OutsideOp(String),
    MemoryInvalidRegister,
}

impl Display for SyntaxErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxErr::TraillingComma => write!(f, "trailling comma"),
            SyntaxErr::UnmatchedParen(_) => write!(f, "unmatched parenthesis"),
            SyntaxErr::UnexpectedChar => write!(f, "unexpected character"),
            SyntaxErr::OutsideOp(kind) => write!(f, "{kind} before opcode"),
            SyntaxErr::MemoryInvalidRegister => write!(f, "invalid register"),
        }
    }
}

impl SyntaxErr {
    pub fn note(&self) -> String {
        match self {
            SyntaxErr::TraillingComma => "remove the final comma".to_string(),
            SyntaxErr::UnmatchedParen(false) => "add ')' after the register name".to_string(),
            SyntaxErr::UnmatchedParen(true) => "add '(' before the register name".to_string(),
            SyntaxErr::UnexpectedChar => "ensure the input is well-formed".to_string(),
            SyntaxErr::OutsideOp(kind) => format!("add '{}'s only after an opcode", kind),
            SyntaxErr::MemoryInvalidRegister => "registers are either xN (N < 32 with no leading 0) or the standard aliases".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RuntimeErr {
    InvalidRegister(String),
    UnexpectedImmediate,
    UnexpectedRegister,
    InvalidOp(String),
    InvalidOpArity(String, usize, usize),
}

impl Display for RuntimeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeErr::InvalidRegister(reg) => write!(f, "invalid register {}", reg),
            RuntimeErr::UnexpectedImmediate => write!(f, "unexpected immediate"),
            RuntimeErr::UnexpectedRegister => write!(f, "unexpected register"),
            RuntimeErr::InvalidOp(op) => write!(f, "invalid operation {}", op),
            RuntimeErr::InvalidOpArity(op, expected, actual) => write!(
                f,
                "invalid operation arity {} expected {} got {}",
                op, expected, actual
            ),
        }
    }
}
