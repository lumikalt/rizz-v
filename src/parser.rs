/// TODO: Strings, Symbols
use crate::{env::Env, err::SyntaxErr};
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub enum Token {
    /// ' ', '\t', '\r'
    Spacing,
    /// \# blablabla,
    Comment,
    /// 1, 2, -1
    Immediate(u32),
    /// zero, r1, pc
    ///
    /// Technically also label references and symbols, but we'll handle those later
    Register(String),
    /// add, xor, j
    Op(String, Vec<(Token, Loc)>),
    /// <label>:
    LabelDef(String),
    /// 0(a0)
    Memory(Box<Token>, Option<Box<Token>>),
    /// symbol
    Symbol(String),
    /// "string"
    String(String),

    /// Error token
    Error(ParseErr),
}

impl Token {
    pub fn kind(&self) -> &'static str {
        use Token::*;
        match self {
            Spacing => "spacing",
            Comment => "comment",
            Immediate(_) => "immediate",
            Register(_) => "register",
            Op(_, _) => "op",
            LabelDef(_) => "label def",
            Memory(_, _) => "memory",
            Symbol(_) => "symbol",
            String(_) => "string",
            Error(_) => "error",
        }
    }
}

type ParseErr = (SyntaxErr, Loc, Vec<(Token, Loc)>, Option<String>);

#[derive(Debug, Clone, Copy)]
pub struct Loc {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

fn parse_line(env: &Env, input: &str, line: usize, loc: &mut Loc) -> Result<Vec<(Token, Loc)>, ParseErr> {
    let mut tokens: Vec<(Token, Loc)> = Vec::new();
    let mut chars = input.chars().peekable();

    use Token::*;

    loc.line = line;

    while let Some(c) = chars.next() {
        let token = match c {
            '\t' | ' ' => Spacing,

            '#' => {
                while let Some(_) = chars.peek() {
                    chars.next();
                    loc.end += 1;
                }
                Comment
            }

            '0'..='9' => {
                let mut num = c.to_string();
                while let Some('0'..='9') = chars.peek() {
                    num.push(chars.next().unwrap());
                    loc.end += 1;
                }
                if let Some('(') | Some(' ') | None = chars.peek() {
                    Immediate(num.parse().unwrap())
                } else {
                    return Err((
                        SyntaxErr::UnexpectedChar,
                        Loc {
                            line,
                            start: loc.end + 1,
                            end: loc.end + 1,
                        },
                        tokens.clone(),
                        None,
                    ));
                }
            }
            '-' => {
                let mut num = c.to_string();
                while let Some('0'..='9') = chars.peek() {
                    num.push(chars.next().unwrap());
                }
                Immediate(num.parse().unwrap())
            }
            '(' => {
                let start = loc.start + 2;

                let imm;
                if let Some((Immediate(_), _)) = tokens.last() {
                    imm = Box::new(tokens.pop().unwrap());
                    loc.start = imm.1.start;
                } else {
                    return Err((
                        SyntaxErr::UnexpectedChar,
                        loc.clone(),
                        tokens.clone(),
                        Some("a memory index must be of the form imm(reg) or imm".to_string()),
                    ));
                }

                let mut reg = std::string::String::new();
                while let Some(' ') | Some('0'..='9') | Some('a'..='z') | Some('A'..='Z') =
                    chars.peek()
                {
                    reg.push(chars.next().unwrap());
                    loc.end += 1;
                }
                let end = loc.end + 1;

                let reg = reg.trim();
                if !env.is_valid_register(reg) {
                    return Err((
                        SyntaxErr::MemoryInvalidRegister,
                        Loc { line, start, end },
                        tokens.clone(),
                        None,
                    ));
                }
                if chars.next() != Some(')') {
                    return Err((
                        SyntaxErr::UnmatchedParen(false),
                        loc.clone(),
                        tokens.clone(),
                        None,
                    ));
                }
                loc.end += 2;

                Memory(
                    Box::new(imm.0),
                    Some(Box::new(Register(reg.trim().to_string()))),
                )
            }
            ')' => {
                return Err((
                    SyntaxErr::UnmatchedParen(true),
                    loc.clone(),
                    tokens.clone(),
                    None,
                ))
            }

            // Opcode or Label definition
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut str = c.to_string();
                while let Some('a'..='z') | Some('A'..='Z') | Some('_') | Some('0'..='9')
                | Some('.') = chars.peek()
                {
                    str.push(chars.next().unwrap());
                    loc.end += 1;
                }
                if let Some(':') = chars.peek() {
                    chars.next();
                    loc.end += 1;
                    LabelDef(str[..str.len()].to_string())
                } else {
                    // These Registers may actually be ops, label references or symbols, but there's ambiguity
                    // between them and registers, so we'll just assume they're registers for now
                    Register(str.trim().to_owned())
                }
            }
            _ => return Err((SyntaxErr::UnexpectedChar, loc.clone(), tokens.clone(), None)),
        };
        tokens.push((token, loc.clone()));
        loc.end += 1; // Newline
        loc.start = loc.end;
    }

    let tokens = tokens
        .into_iter()
        .filter(|(token, _)| !matches!(token, Token::Spacing))
        .group_by(|(token, _)| {
            matches!(token, Immediate(_) | Register(_) | Memory(_, _) | Symbol(_))
        })
        .into_iter()
        .flat_map(|group| {
            let (is_op, group) = group;
            if is_op {
                let group = group.collect::<Vec<_>>();
                let (op, loc) = group[0].clone();
                let (name, mut args) = match op {
                    Register(r) => (r, vec![]),
                    // because any register/symbol/label def is interpreted as an Op by default, this only
                    // partially works. This does trigger on immediate values and memory indexes
                    _ => {
                        return vec![(
                            Token::Error((
                                SyntaxErr::OutsideOp(op.kind().to_string()),
                                loc.clone(),
                                group.clone(),
                                None,
                            )),
                            loc.clone(),
                        )]
                    }
                };
                if env.is_valid_register(&name) {
                    return vec![(
                        Token::Error((
                            SyntaxErr::OutsideOp("register".to_string()),
                            loc.clone(),
                            group.clone(),
                            None,
                        )),
                        loc.clone(),
                    )];
                }

                args.extend_from_slice(&group[1..]);

                vec![(Op(name, args), loc)]
            } else {
                group.collect::<Vec<_>>()
            }
        })
        .collect::<Vec<_>>();
    if let Some((Token::Error(err), _)) =
        tokens.iter().find(|line| matches!(line.0, Token::Error(_)))
    {
        Err(err.to_owned())
    } else {
        Ok(tokens)
    }
}

/// Parse the input
///
/// Returns a vector of tokens and their locations, if successful, or an error vector
/// containing the error, the location of the error, the tokens parsed up to that point,
/// and an optional message to display to the users for each line with an error
pub fn parse(env: &Env, input: &str) -> Result<Vec<(Token, Loc)>, Vec<ParseErr>> {
    let mut loc = Loc {
        line: 0,
        start: 0,
        end: 0,
    };

    let parsed_lines = input
        .lines()
        .enumerate()
        .map(|(i, line)| parse_line(env, line, i + 1, &mut loc))
        .collect::<Vec<_>>();

    let (ok, err): (Vec<_>, Vec<_>) = parsed_lines.into_iter()
        .partition(|line| matches!(line, Ok(_)));

    dbg!(err.clone());

    if err.is_empty() {
        Ok(ok.into_iter().flat_map(|line| line.unwrap()).collect())
    } else {
        Err(err.into_iter().map(|line| line.unwrap_err()).collect())
    }
}
