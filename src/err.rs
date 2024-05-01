use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
};

use itertools::Itertools;

use crate::instructions::instruction;

#[derive(Debug, Clone)]
pub enum SyntaxErr {
    UnexpectedChar,

    //.text specific
    /// false for '(' true for ')'
    UnmatchedParen(bool),
    OutsideMnemonic(String),
    InvalidRegister,

    // .data specific
    InvalidType,
    InvalidVarName,
    MalformedData,
}

impl Display for SyntaxErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxErr::UnexpectedChar => write!(f, "unexpected character"),
            SyntaxErr::UnmatchedParen(_) => write!(f, "unmatched parenthesis"),
            SyntaxErr::OutsideMnemonic(kind) => write!(f, "unexpected '{kind}'"),
            SyntaxErr::InvalidRegister => write!(f, "invalid register"),
            SyntaxErr::InvalidType => write!(f, "invalid type"),
            SyntaxErr::InvalidVarName => write!(f, "invalid variable name"),
            SyntaxErr::MalformedData => write!(f, "malformed global definition"),
        }
    }
}

impl SyntaxErr {
    pub fn note(&self) -> String {
        match self {
            SyntaxErr::UnexpectedChar => "ensure the input is well-formed".to_string(),
            SyntaxErr::UnmatchedParen(false) => "add `)` after the register".to_string(),
            SyntaxErr::UnmatchedParen(true) => "add `(` before the register".to_string(),
            SyntaxErr::OutsideMnemonic(_) => format!("only add arguments after the mnemonic"),
            SyntaxErr::InvalidRegister => {
                "registers are either (x|f)N, for N < 32 with no leading 0, or an alias".to_string()
            }
            SyntaxErr::InvalidType => "check the spec for proper types".to_string(),
            SyntaxErr::InvalidVarName => "variable names must be alphanumeric".to_string(),
            SyntaxErr::MalformedData => "ensure the global definition is well-formed".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RuntimeErr {
    /// TODO: only worth using this after all the instructions are implemented!
    InvalidMnemonic,
    /// op, actual, expected
    InvalidOpArity(String, usize, usize),
    /// actual, expected
    TypeMissmatch(String, String),
    LabelNotFound,
}

impl Display for RuntimeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeErr::InvalidMnemonic => write!(f, "invalid mnemonic"),
            RuntimeErr::InvalidOpArity(_, actual, expected) => {
                write!(f, "expected {} args, got {}", expected, actual)
            }
            RuntimeErr::TypeMissmatch(actual, expected) => {
                write!(f, "expected '{}', got '{}'", expected, actual)
            }
            RuntimeErr::LabelNotFound => write!(f, "label not found"),
        }
    }
}

impl RuntimeErr {
    pub fn note(&self) -> String {
        match self {
            RuntimeErr::InvalidMnemonic => {
                "check the ref sheet for the avaliable mnemonics".to_string()
            }
            RuntimeErr::InvalidOpArity(op, actual, expected) => {
                let args = instruction(op).unwrap().1;
                match actual.cmp(expected) {
                    Ordering::Equal => unreachable!(),
                    Ordering::Greater if actual - expected == 1 => {
                        "remove the extra argument".to_string()
                    }
                    Ordering::Greater => "remove the extra arguments".to_string(),
                    Ordering::Less if expected - actual == 1 => {
                        format!("add the extra '{}' argument", args.last().unwrap().kind())
                    }
                    Ordering::Less => format!(
                        "add the extra `{}` arguments",
                        args.get((actual - 1)..)
                            .unwrap()
                            .iter()
                            .map(|arg| arg.kind())
                            .join("', '")
                    ),
                }
            }
            RuntimeErr::TypeMissmatch(_, _) => {
                "ensure the instruction is getting the right arguments".to_string()
            }
            RuntimeErr::LabelNotFound => "ensure the label is spelled correctly".to_string(),
        }
    }
}
