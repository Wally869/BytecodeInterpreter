#[derive(Copy, Clone, Debug)]
pub enum ByteCode {
    LoadVal(i32),
    WriteVar(char),
    ReadVar(char),
    Add,
    Sub,
    Mul,
    Div,
    Return,

    // Comparisons
    Greater,
    Lower,
    Eq,
    GreaterEq,
    LowerEq,

    // Jumps with target location
    Jump(u32),
    JumpI(u32), // jump if
}
