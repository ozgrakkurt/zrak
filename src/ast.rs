use crate::str_interner::IntStr;
use crate::token::{Assign, CmpOp, FactorOp, ShiftOp, TermOp};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Program {
    pub decls: Vec<Decl>,
}

#[derive(Debug)]
pub enum Decl {
    Struct(StructDecl),
    Fun(FunDecl),
    Var(VarDecl),
    Stmt(Stmt),
}

#[derive(Debug)]
pub struct StructDecl {
    pub ident: IntStr,
    pub methods: HashMap<IntStr, FunDecl>,
}

#[derive(Debug)]
pub struct FunDecl {
    pub ident: IntStr,
    pub params: Vec<IntStr>,
    pub block: Block,
}

#[derive(Debug)]
pub struct VarDecl {
    pub ident: IntStr,
    pub expr: Expr,
}

#[derive(Debug)]
pub enum Stmt {
    Return(Option<Expr>),
    Break(Option<Expr>),
    Assignment(Assignment),
    Expr(Expr),
}

#[derive(Debug)]
pub struct Assignment {
    pub lcall: LCall,
    pub assigner: Assign,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct LCall {
    pub head: LCallHead,
    pub tail: Vec<LCallPart>,
}

#[derive(Debug)]
pub enum LCallHead {
    Ident(IntStr),
    SelfKw,
}

#[derive(Debug)]
pub enum LCallPart {
    Dot(IntStr),
    Brkts(Box<Expr>),
}

#[derive(Debug)]
pub struct Expr {
    pub logic_or: LogicOr,
}

#[derive(Debug)]
pub enum LogicOr {
    Next(LogicAnd),
    Current(LogicAnd, Box<LogicOr>),
}

#[derive(Debug)]
pub enum LogicAnd {
    Next(Cmp),
    Current(Cmp, Box<LogicAnd>),
}

#[derive(Debug)]
pub enum Cmp {
    Next(BitOr),
    Current {
        left: BitOr,
        op: CmpOp,
        cmp: Box<Cmp>,
    },
}

#[derive(Debug)]
pub enum BitOr {
    Next(BitXor),
    Current(BitXor, Box<BitOr>),
}

#[derive(Debug)]
pub enum BitXor {
    Next(BitAnd),
    Current(BitAnd, Box<BitXor>),
}

#[derive(Debug)]
pub enum BitAnd {
    Next(Shift),
    Current(Shift, Box<BitAnd>),
}

#[derive(Debug)]
pub enum Shift {
    Next(Term),
    Current {
        left: Term,
        op: ShiftOp,
        shift: Box<Shift>,
    },
}

#[derive(Debug)]
pub enum Term {
    Next(Factor),
    Current {
        left: Factor,
        op: TermOp,
        term: Box<Term>,
    },
}

#[derive(Debug)]
pub enum Factor {
    Next(Unary),
    Current {
        left: Unary,
        op: FactorOp,
        factor: Box<Factor>,
    },
}

#[derive(Debug)]
pub enum Unary {
    Next(Call),
    Current { op: UnaryOp, unary: Box<Unary> },
}

#[derive(Debug)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug)]
pub struct Call {
    pub head: Primary,
    pub tail: Vec<CallPart>,
}

#[derive(Debug)]
pub enum CallPart {
    Dot(IntStr),
    Brkts(Box<Expr>),
    FunCall(Vec<Expr>),
    QMark,
}

#[derive(Debug)]
pub enum Primary {
    SelfKw,
    Prnth(Box<Expr>),
    Ident(IntStr),
    For(For),
    While(While),
    Loop(Loop),
    If(If),
    Closure(Closure),
    Block(Block),
    Literal(Literal),
}

#[derive(Debug)]
pub struct For {
    pub ident: IntStr,
    pub expr: Box<Expr>,
    pub block: Block,
}

#[derive(Debug)]
pub struct While {
    pub cond: Box<Expr>,
    pub block: Block,
}

#[derive(Debug)]
pub struct Loop {
    pub block: Block,
}

#[derive(Debug)]
pub struct If {
    pub cond: Box<Expr>,
    pub block: Block,
    pub els: Option<Else>,
}

#[derive(Debug)]
pub enum Else {
    If(Box<If>),
    Block(Block),
}

#[derive(Debug)]
pub struct Closure {
    pub params: Vec<IntStr>,
    pub block: Block,
}

#[derive(Debug)]
pub struct Block {
    pub decls: Vec<Decl>,
    pub expr: Option<Box<Expr>>,
}

#[derive(Debug)]
pub enum Literal {
    Bool(bool),
    Null,
    Int(i64),
    Float(f64),
    Char(char),
    Str(IntStr),
    Struct(StructLit),
    Map(MapLit),
    Array(ArrayLit),
}

#[derive(Debug)]
pub struct StructLit {
    pub ident: IntStr,
    pub fields: Vec<(IntStr, Expr)>,
}

#[derive(Debug)]
pub struct MapLit {
    pub fields: Vec<(Expr, Expr)>,
}

#[derive(Debug)]
pub struct ArrayLit {
    pub elems: Vec<Expr>,
}
