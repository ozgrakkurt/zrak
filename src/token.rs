use crate::str_interner::IntStr;

pub enum Token {
    Assign(Assign),
    Operator(Operator),
    Ident(IntStr),
    Literal(Literal),
    Delimiter(Delimiter),
}

pub enum Literal {
    Bool(bool),
    Null,
    Int(i64),
    Float(f64),
    Char(char),
    Str(IntStr),
}

pub enum Keyword {
    In,
    For,
    While,
    Loop,
    If,
    Else,
    Struct,
    Fn,
    Let,
    SelfKw,
    Return,
    Break,
}

pub enum Delimiter {
    OpenBrkt,
    CloseBrkt,
    OpenPrnth,
    ClosePrnth,
    OpenCurly,
    CloseCurly,
    Dot,
    Comma,
}

pub enum Assign {
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    LeftShift,
    RightShift,
}

pub enum Operator {
    LogicOr,
    LogicAnd,
    Cmp(CmpOp),
    BitOr,
    BitXor,
    BitAnd,
    LeftShift,
    RightShift,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Not,
    QMark,
}

pub enum CmpOp {
    Eq,
    NotEq,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}
