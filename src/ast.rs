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
    pub ident: String,
    pub fns: Vec<FunDecl>,
}

pub struct FunDecl {
    pub ident: String,
    pub params: Vec<String>,
    pub blk: Block,
}

pub struct VarDecl {
    pub ident: String,
    pub expr: Expr,
}

pub enum Stmt {
    Return(Option<Expr>),
    Break(Option<Expr>),
    Assignment(Assignment),
    Expr(Expr),
}

pub struct Assignment {
    pub call: Call,
    pub op: AssignOp,
    pub expr: Expr,
}

pub enum AssignOp {
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
    LogicAnd,
    LogicOr,
}

pub struct Expr {
    pub or: LogicOr,
}

pub enum LogicOr {
    A(LogicAnd),
    B { left: Box<LogicOr>, right: LogicAnd },
}

pub enum LogicAnd {
    A(Cmp),
    B { left: Box<LogicAnd>, right: Cmp },
}

pub enum Cmp {
    A(BitOr),
    B {
        left: Box<Cmp>,
        op: CmpOp,
        right: BitOr,
    },
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
    B { left: Box<BitOr>, right: BitXor },
}

pub enum BitXor {
    A(BitAnd),
    B { left: Box<BitXor>, right: BitAnd },
}

pub enum BitAnd {
    A(Shift),
    B { left: Box<BitAnd>, right: Shift },
}

pub enum Shift {
    A(Term),
    B {
        left: Box<Shift>,
        op: ShiftOp,
        right: Term,
    },
}

pub enum ShiftOp {
    Left,
    Right,
}

pub enum Term {
    A(Factor),
    B {
        left: Box<Term>,
        op: TermOp,
        right: Factor,
    },
}

pub enum TermOp {
    Add,
    Sub,
}

pub enum Factor {
    A(TypeCast),
    B {
        left: Box<Factor>,
        op: FactorOp,
        right: TypeCast,
    },
}

pub enum FactorOp {
    Mul,
    Div,
    Mod,
}

pub enum TypeCast {
    A(Unary),
    B {
        left: Box<TypeCast>,
        type_name: TypeName,
    },
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
    A(Call),
    B { op: UnaryOp, unary: Box<Unary> },
}

pub enum UnaryOp {
    Minus,
    Not,
}

pub struct Call {
    pub primary: Primary,
    pub call: Vec<CallPart>,
}

pub enum CallPart {
    Ident(String),
    Brkts(Box<Expr>),
    FunCall(Vec<Expr>),
    QMark,
}

pub enum Primary {
    Prnth(Box<Expr>),
    Literal(Literal),
    This,
    Ident(String),
    For(For),
    While(While),
    Loop(Loop),
    If(If),
    Block(Block),
}

pub struct For {
    pub ident: String,
    pub expr: Box<Expr>,
    pub blk: Block,
}

pub struct While {
    pub expr: Box<Expr>,
    pub blk: Block,
}

pub struct Loop {
    pub blk: Block,
}

pub struct If {
    pub cond: Box<Expr>,
    pub blk: Block,
    pub els: Option<Else>,
}

pub enum Else {
    Block(Block),
    If(Box<If>),
}

pub enum Literal {
    Bool(bool),
    Null,
    Int(i64),
    Float(f64),
    Str(String),
    Char(char),
    Map(Vec<MapField>),
    Array(Vec<Expr>),
    Struct(StructLit),
    Closure(Closure),
}

pub struct MapField {
    pub key: String,
    pub val: Box<Expr>,
}

pub struct StructLit {
    pub ident: String,
    pub fields: Vec<StructField>,
}

pub struct StructField {
    pub ident: String,
    pub expr: Box<Expr>,
}

pub struct Closure {
    pub params: Vec<String>,
    pub blk: Block,
}

pub struct Block {
    pub decls: Vec<Decl>,
    pub expr: Box<Option<Expr>>,
}
