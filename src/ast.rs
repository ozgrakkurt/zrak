pub struct Program {
    decls: Vec<Decl>,
}

pub enum Decl {
    Struct(StructDecl),
    Fun(FunDecl),
    Var(VarDecl),
    Stmt(Stmt),
}

pub struct StructDecl {
    ident: String,
    fns: Vec<FunDecl>,
}

pub struct FunDecl {
    ident: String,
    params: Vec<String>,
    blk: Block,
}

pub struct VarDecl {
    ident: String,
    expr: Expr,
}

pub enum Stmt {
    Return(Option<Expr>),
    Break(Option<Expr>),
    Assignment(Assignment),
    Expr(Expr),
}

pub struct Assignment {
    call: Call,
    expr: Expr,
}

pub struct Expr {
    or: LogicOr,
}

pub enum LogicOr {
    A(LogicAnd),
    B {
        left: LogicOr,
        right: LogicAnd,
    }
}

pub enum LogicAnd {
    A(Cmp),
    B {
        left: LogicAnd,
        right: Cmp,
    }
}

pub enum Cmp {
    A(BitOr),
    B {
        left: Cmp,
        op: CmpOp,
        right: BitOr,
    }
}

pub enum CmpOp {
    Eq,
    NotEq,
    Less,
    Grt,
    LessEq,
    GrtEq,
}

pub enum BitOr {
    A(BitXor),
    B {
        left: BitOr,
        right: BitXor,
    }
}

pub enum BitXor {
    A(BitAnd),
    B {
        left: BitXor,
        right: BitAnd,
    }
}

pub enum BitAnd {
    A(Shift),
    B {
        left: BitAnd,
        right: Shift,
    }
}

pub enum Shift {
    A(Term),
    B {
        left: Shift,
        op: ShiftOp,
        right: Term,
    }
}

pub enum ShiftOp {
    Left,
    Right,
}

pub enum Term {
    A(Factor),
    B {
        left: Term,
        op: TermOp,
        right: Factor,
    }
}

pub enum TermOp {
    Add,
    Sub,
}

pub enum Factor {
    A(TypeCast),
    B {
        left: Factor,
        op: FactorOp,
        right: TypeCast,
    }
}

pub enum FactorOp {
    Mul,
    Div,
    Mod,
}

pub enum TypeCast {
    A(Unary),
    B {
        left: TypeCast,
        type_name: TypeName,
    }
}

pub enum TypeName {
    Bool,
    Int,
    Float,
    Function,
    Closure,
    Map,
    Array,
    Ident(String),
}

pub enum Unary {
}
