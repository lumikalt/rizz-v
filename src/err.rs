use std::fmt::{self, Display, Formatter};

pub enum SyntaxErr {
    TraillingComma,
    UnmatchedParen,
}

impl Display for SyntaxErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxErr::TraillingComma => write!(f, "trailling comma"),
            SyntaxErr::UnmatchedParen => write!(f, "unmatched parenthesis"),
        }
    }
}

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
