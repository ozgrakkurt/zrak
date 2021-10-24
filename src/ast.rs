use crate::str_interner::IntStr;

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
    ident: IntStr,
    methods: HashMap<IntStr, FnDecl>,
}

pub struct FnDecl {
    ident: IntStr,
    params: Vec<IntStr>,
    block: Block,
}

pub struct VarDecl {
    ident: IntStr,
    expr: Expr,
}
