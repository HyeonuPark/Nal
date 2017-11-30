use super::{Ast, Ident, Pattern, Stmt, Expr};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Function {
    name: Option<Ast<Ident>>,
    params: Option<Vec<Ast<Pattern>>>,
    body: FunctionBody,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum FunctionBody {
    Stmt(Vec<Ast<Stmt>>),
    Expr(Ast<Expr>),
}
