use super::common::{Ast, Ident};
use super::expr::Expr;
use super::function::Function;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Stmt {
    /// If(condition, positive case, negative case)
    If(Ast<Expr>, Ast<StmtBlock>, Option<Ast<StmtBlock>>),
    /// While(condition, body)
    While(Ast<Expr>, Ast<StmtBlock>),
    /// ForIn(each element, container, body)
    ForIn(Ast<Pattern>, Ast<Expr>, Ast<StmtBlock>),
    /// Function stmt should has it's name
    /// Function(is static, function)
    Function(bool, Ast<Function>),
    /// Let(variables, target value)
    Let(Ast<Pattern>, Ast<Expr>),
    /// Assignment should operate to existing mutable variables
    /// Assign(variables, target value)
    Assign(Ast<Pattern>, Ast<Expr>),
    Expr(Ast<Expr>),
}

pub type StmtBlock = Vec<Ast<Stmt>>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Pattern {
    /// Ident(variable name, is mutable)
    Ident(Ast<Ident>, bool),
    /// Obj([(property name, subpattern)])
    Obj(Vec<(Ast<Ident>, Ast<Pattern>)>),
}
