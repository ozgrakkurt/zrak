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
    lcall: LCall,
    expr: Expr,
}

pub enum Expr {
    For(For),
    While(While),
    Loop(Loop),
    If(If),
    Block(Block),
    LogicOr(LogicOr),
}
