/// TODO: Strings, Symbols
use crate::{env::Env, err::SyntaxErr};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum Token {
    /// ' ', '\t', '\r', \# blablabla
    Spacing,
    /// 1, 2, -1
    Immediate(u32),
    /// zero, r1, pc
    ///
    /// Technically also label references and symbols, but we'll handle those later
    Register(String),
    /// add, xor, j
    Op(String, Vec<(Token, Loc)>),
    /// \<label>:
    Label(String),
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
            Immediate(_) => "immediate",
            Register(_) => "register",
            Op(_, _) => "op",
            Label(_) => "label",
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
    pub mem_offset: usize,
}

impl Default for Loc {
    fn default() -> Self {
        Self {
            line: 0,
            start: 0,
            end: 0,
            mem_offset: 0,
        }
    }
}

fn parse_line(env: &Env, input: &str, loc: &mut Loc) -> Result<Vec<(Token, Loc)>, ParseErr> {
    let mut tokens: Vec<(Token, Loc)> = Vec::new();
    let mut chars = input.chars().peekable();

    use Token::*;

    while let Some(c) = chars.next() {
        let token = match c {
            '\t' | ' ' => Spacing,

            '#' => {
                while let Some(_) = chars.peek() {
                    chars.next();
                    loc.end += 1;
                }
                Spacing
            }

            '0' if chars.peek() == Some(&'x') => {
                chars.next();
                loc.end += 1;
                let mut num = std::string::String::new();
                while let Some('0'..='9') | Some('a'..='f') | Some('A'..='F') = chars.peek() {
                    num.push(chars.next().unwrap());
                    loc.end += 1;
                }
                if let Some('(') | Some(' ') | None = chars.peek() {
                    Immediate(u32::from_str_radix(&num, 16).unwrap())
                } else {
                    let err = Err((
                        SyntaxErr::UnexpectedChar,
                        Loc {
                            start: loc.end + 1,
                            end: loc.end + 1,
                            ..*loc
                        },
                        tokens.clone(),
                        None,
                    ));
                    advance_to_next_line(&mut chars, loc);
                    return err;
                }
            }
            '0' if chars.peek() == Some(&'b') => {
                chars.next();
                loc.end += 1;
                let mut num = std::string::String::new();
                while let Some('0'..='1') = chars.peek() {
                    num.push(chars.next().unwrap());
                    loc.end += 1;
                }
                if let Some('(') | Some(' ') | None = chars.peek() {
                    Immediate(u32::from_str_radix(&num, 2).unwrap())
                } else {
                    let err = Err((
                        SyntaxErr::UnexpectedChar,
                        Loc {
                            start: loc.end + 1,
                            end: loc.end + 1,
                            ..*loc
                        },
                        tokens.clone(),
                        None,
                    ));
                    advance_to_next_line(&mut chars, loc);
                    return err;
                }
            }
            '0' if chars.peek() == Some(&'o') => {
                chars.next();
                loc.end += 1;
                let mut num = std::string::String::new();
                while let Some('0'..='7') = chars.peek() {
                    num.push(chars.next().unwrap());
                    loc.end += 1;
                }
                if let Some('(') | Some(' ') | None = chars.peek() {
                    Immediate(u32::from_str_radix(&num, 8).unwrap())
                } else {
                    let err = Err((
                        SyntaxErr::UnexpectedChar,
                        Loc {
                            start: loc.end + 1,
                            end: loc.end + 1,
                            ..*loc
                        },
                        tokens.clone(),
                        None,
                    ));
                    advance_to_next_line(&mut chars, loc);
                    return err;
                }
            }
            '0'..='9' => {
                let mut num = c.to_string();
                while let Some('0'..='9') = chars.peek() {
                    num.push(chars.next().unwrap());
                    loc.end += 1;
                }
                if let Some('(') | Some(' ') | None = chars.peek() {
                    dbg!(Immediate(num.parse().unwrap()))
                } else {
                    let err = Err((
                        SyntaxErr::UnexpectedChar,
                        Loc {
                            start: loc.end + 1,
                            end: loc.end + 1,
                            ..*loc
                        },
                        tokens.clone(),
                        None,
                    ));
                    advance_to_next_line(&mut chars, loc);
                    return err;
                }
            }
            '-' => {
                let mut num = c.to_string();
                while let Some('0'..='9') = chars.peek() {
                    num.push(chars.next().unwrap());
                    loc.end += 1;
                }
                Immediate(num.parse::<i32>().unwrap() as u32)
            }
            '(' => {
                let start = loc.start + 2;

                let imm;
                if let Some((Immediate(_), _)) = tokens.last() {
                    imm = Box::new(tokens.pop().unwrap());
                    loc.start = imm.1.start;
                } else {
                    let err = Err((
                        SyntaxErr::UnexpectedChar,
                        loc.clone(),
                        tokens.clone(),
                        Some("a memory index must be of the form imm(reg) or imm".to_string()),
                    ));
                    advance_to_next_line(&mut chars, loc);
                    return err;
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
                if env.str_to_register(reg).is_none() {
                    let err = Err((
                        SyntaxErr::InvalidRegister,
                        Loc {
                            start,
                            end,
                            ..*loc
                        },
                        tokens.clone(),
                        None,
                    ));
                    advance_to_next_line(&mut chars, loc);
                    return err;
                }
                if chars.next() != Some(')') {
                    let err = Err((
                        SyntaxErr::UnmatchedParen(false),
                        loc.clone(),
                        tokens.clone(),
                        None,
                    ));
                    advance_to_next_line(&mut chars, loc);
                    return err;
                }
                loc.end += 2;

                Memory(
                    Box::new(imm.0),
                    Some(Box::new(Register(reg.trim().to_string()))),
                )
            }
            ')' => {
                let err = Err((
                    SyntaxErr::UnmatchedParen(true),
                    loc.clone(),
                    tokens.clone(),
                    None,
                ));
                advance_to_next_line(&mut chars, loc);
                return err;
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
                    Label(str[..str.len()].to_string())
                } else {
                    // These Registers may actually be ops, label references or symbols, but there's ambiguity
                    // between them and registers, so we'll just assume they're registers for now
                    Register(str.trim().to_owned())
                }
            }
            _ => {
                let err = Err((SyntaxErr::UnexpectedChar, loc.clone(), tokens.clone(), None));
                advance_to_next_line(&mut chars, loc);
                return err;
            }
        };
        tokens.push((token, loc.clone()));
        loc.end += 1;
        loc.start = loc.end;
    }

    loc.end += 1; // Newline
    loc.start = loc.end;

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
                                SyntaxErr::OutsideMnemonic(op.kind().to_string()),
                                loc.clone(),
                                group.clone(),
                                None,
                            )),
                            loc.clone(),
                        )]
                    }
                };
                if env.str_to_register(&name).is_some() {
                    return vec![(
                        Token::Error((
                            SyntaxErr::OutsideMnemonic("register".to_string()),
                            loc.clone(),
                            group.clone(),
                            None,
                        )),
                        loc.clone(),
                    )];
                }
                for (token, loc) in group[1..].iter() {
                    match token.clone() {
                        Token::Register(name) => {
                            if env.str_to_register(&name).is_some() {
                                args.push((token.clone(), loc.clone()));
                            } else {
                                args.push((Token::Symbol(name.to_owned()), *loc))
                            }
                        }
                        others => args.push((others, *loc)),
                    }
                }

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
        mem_offset: 0,
    };

    let parsed_lines = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            loc.line = i + 1;
            parse_line(env, line, &mut loc)
        })
        .collect::<Vec<_>>();

    let (ok, err): (Vec<_>, Vec<_>) = parsed_lines
        .into_iter()
        .partition(|line| matches!(line, Ok(_)));

    if err.is_empty() {
        Ok(ok.into_iter().flat_map(|line| line.unwrap()).collect())
    } else {
        Err(err.into_iter().map(|line| line.unwrap_err()).collect())
    }
}

fn advance_to_next_line(chars: &mut std::iter::Peekable<std::str::Chars>, loc: &mut Loc) {
    while let Some(_) = chars.peek() {
        chars.next();
        loc.end += 1;
    }
    loc.end += 1; // Newline
    loc.start = loc.end;
}
