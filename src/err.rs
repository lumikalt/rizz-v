use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
};

use itertools::Itertools;

use crate::instructions::instruction;

#[derive(Debug, Clone)]
pub enum SyntaxErr {
    /// false for '(' true for ')'
    UnmatchedParen(bool),
    UnexpectedChar,
    OutsideOp(String),
    MemoryInvalidRegister,
}

impl Display for SyntaxErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxErr::UnmatchedParen(_) => write!(f, "unmatched parenthesis"),
            SyntaxErr::UnexpectedChar => write!(f, "unexpected character"),
            SyntaxErr::OutsideOp(kind) => write!(f, "`{kind}` before opcode"),
            SyntaxErr::MemoryInvalidRegister => write!(f, "invalid register"),
        }
    }
}

impl SyntaxErr {
    pub fn note(&self) -> String {
        match self {
            SyntaxErr::UnmatchedParen(false) => "add `)` after the register name".to_string(),
            SyntaxErr::UnmatchedParen(true) => "add `(` before the register name".to_string(),
            SyntaxErr::UnexpectedChar => "ensure the input is well-formed".to_string(),
            SyntaxErr::OutsideOp(_) => format!("only add arguments after the opcode"),
            SyntaxErr::MemoryInvalidRegister => {
                "registers are either xN (N < 32 with no leading 0) or the standard aliases"
                    .to_string()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum RuntimeErr {
    InvalidOp,
    /// op, actual, expected
    InvalidOpArity(String, usize, usize),
    /// actual, expected
    InvalidType(String, String),
}

impl Display for RuntimeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeErr::InvalidOp => write!(f, "invalid opcode"),
            RuntimeErr::InvalidOpArity(op, actual, expected) => {
                write!(f, "`{}` expected {} args, got {}", op, expected, actual)
            }
            RuntimeErr::InvalidType(actual, expected) => {
                write!(f, "expected `{}`, got `{}`", expected, actual)
            }
        }
    }
}

impl RuntimeErr {
    pub fn note(&self) -> String {
        match self {
            RuntimeErr::InvalidOp => "check the ref sheet for the avaliable opcodes".to_string(),
            RuntimeErr::InvalidOpArity(op, actual, expected) => {
                let args = instruction(op).unwrap().1;
                match actual.cmp(expected) {
                    Ordering::Equal => unreachable!(),
                    Ordering::Greater if actual - expected == 1 => {
                        "remove the extra argument".to_string()
                    }
                    Ordering::Greater => "remove the extra arguments".to_string(),
                    Ordering::Less if expected - actual == 1 => {
                        format!("add the extra `{}` argument", args.last().unwrap().kind())
                    }
                    Ordering::Less => format!(
                        "add the extra `{}` arguments",
                        args.get((actual - 1)..)
                            .unwrap()
                            .iter()
                            .map(|arg| arg.kind())
                            .join("`, `")
                    ),
                }
            }
            RuntimeErr::InvalidType(_, _) => "ensure the operation is valid".to_string(),
        }
    }
}
