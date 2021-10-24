use crate::str_interner::IntStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Assign(Assign),
    Operator(Operator),
    Ident(IntStr),
    Literal(Literal),
    Delimiter(Delimiter),
    Keyword(Keyword),
    Eof,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Literal {
    Bool(bool),
    Null,
    Int(i64),
    Float(f64),
    Char(char),
    Str(IntStr),
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    Map,
    New,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Delimiter {
    OpenBrkt,
    CloseBrkt,
    OpenPrnth,
    ClosePrnth,
    OpenCurly,
    CloseCurly,
    Dot,
    Comma,
    Colon,
    Semicolon,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    LogicOr,
    LogicAnd,
    Cmp(CmpOp),
    BitOr,
    BitXor,
    BitAnd,
    Shift(ShiftOp),
    Term(TermOp),
    Factor(FactorOp),
    Not,
    QMark,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TermOp {
    Add,
    Sub,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FactorOp {
    Mul,
    Div,
    Mod,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShiftOp {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CmpOp {
    Eq,
    NotEq,
    Less,
    Greater,
    LessEq,
    GreaterEq,
}
