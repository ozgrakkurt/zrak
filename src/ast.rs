use crate::str_interner::IntStr;
use crate::token::{Assign, CmpOp, FactorOp, ShiftOp, TermOp};
use std::collections::HashMap;

pub struct Program {
    pub decls: Vec<Decl>,
}

pub enum Decl {
    Struct(StructDecl),
    Fun(FunDecl),
    Var(VarDecl),
    Stmt(Stmt),
}

pub struct StructDecl {
    pub ident: IntStr,
    pub methods: HashMap<IntStr, FunDecl>,
}

pub struct FunDecl {
    pub ident: IntStr,
    pub params: Vec<IntStr>,
    pub block: Block,
}

pub struct VarDecl {
    pub ident: IntStr,
    pub expr: Expr,
}

pub enum Stmt {
    Return(Option<Expr>),
    Break(Option<Expr>),
    Assignment(Assignment),
    Expr(Expr),
}

pub struct Assignment {
    pub lcall: LCall,
    pub assigner: Assign,
    pub expr: Expr,
}

pub struct LCall {
    pub head: LCallHead,
    pub tail: Vec<LCallPart>,
}

pub enum LCallHead {
    Ident(IntStr),
    SelfKw,
}

pub enum LCallPart {
    Dot(IntStr),
    Brkts(Box<Expr>),
}

pub struct Expr {
    pub logic_or: LogicOr,
}

pub enum LogicOr {
    Next(LogicAnd),
    Current(LogicAnd, Box<LogicOr>),
}

pub enum LogicAnd {
    Next(Cmp),
    Current(Cmp, Box<LogicAnd>),
}

pub enum Cmp {
    Next(BitOr),
    Current {
        left: BitOr,
        op: CmpOp,
        cmp: Box<Cmp>,
    },
}

pub enum BitOr {
    Next(BitXor),
    Current(BitXor, Box<BitOr>),
}

pub enum BitXor {
    Next(BitAnd),
    Current(BitAnd, Box<BitXor>),
}

pub enum BitAnd {
    Next(Shift),
    Current(Shift, Box<BitAnd>),
}

pub enum Shift {
    Next(Term),
    Current {
        left: Term,
        op: ShiftOp,
        shift: Box<Shift>,
    },
}

pub enum Term {
    Next(Factor),
    Current {
        left: Factor,
        op: TermOp,
        term: Box<Term>,
    },
}

pub enum Factor {
    Next(Unary),
    Current {
        left: Unary,
        op: FactorOp,
        factor: Box<Factor>,
    },
}

pub enum Unary {
    Next(Call),
    Current { op: UnaryOp, unary: Box<Unary> },
}

pub enum UnaryOp {
    Negate,
    Not,
}

pub struct Call {
    pub head: Primary,
    pub tail: Vec<CallPart>,
}

pub enum CallPart {
    Dot(IntStr),
    Brkts(Box<Expr>),
    FunCall(Vec<Expr>),
    QMark,
}

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

pub struct For {
    pub ident: IntStr,
    pub expr: Box<Expr>,
    pub block: Block,
}

pub struct While {
    pub cond: Box<Expr>,
    pub block: Block,
}

pub struct Loop {
    pub block: Block,
}

pub struct If {
    pub cond: Box<Expr>,
    pub block: Block,
    pub els: Option<Else>,
}

pub enum Else {
    If(Box<If>),
    Block(Block),
}

pub struct Closure {
    pub params: Vec<IntStr>,
    pub block: Block,
}

pub struct Block {
    pub decls: Vec<Decl>,
    pub expr: Option<Box<Expr>>,
}

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

pub struct StructLit {
    pub ident: IntStr,
    pub fields: Vec<(IntStr, Expr)>,
}

pub struct MapLit {
    pub ident: IntStr,
    pub fields: Vec<(Expr, Expr)>,
}

pub struct ArrayLit {
    pub elems: Vec<Expr>,
}
