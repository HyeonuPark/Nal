use super::{Ast, Ident, Stmt, Pattern, Expr, NamedLiteral};

#[derive(Debug)]
pub struct HashBang(String);

#[derive(Debug)]
pub struct Import {
    targets: Vec<Ast<Ident>>,
    source: String,
}

#[derive(Debug)]
pub enum TopLevelStmt {
    Stmt(Ast<Stmt>),
    ExportLet(Ast<Pattern>, Ast<Expr>),
    ExportNamed(Ast<NamedLiteral>),
}

#[derive(Debug)]
pub struct Module {
    hashbang: Option<Ast<HashBang>>,
    imports: Vec<Ast<Import>>,
    body: Vec<Ast<TopLevelStmt>>,
}
