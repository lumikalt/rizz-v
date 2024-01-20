pub enum Expr {
    /// 1, 2, -1
    Immediate(i64),
    /// zero, r1, pc
    Register(String),
    /// add, xor, j
    Op(Vec<Expr>),
    /// <label>:
    LabelDef(String),
    /// j <label>
    LabelRef(String),
}

pub struct Location {
    pub line: usize,
    pub col: usize,
    pub start: usize,
    pub end: usize,
}

pub fn parse(input: &str) -> Result<Expr, ()> {
    todo!()
}
