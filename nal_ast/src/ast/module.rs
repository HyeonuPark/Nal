use super::common::Ast;
use super::stmt::Stmt;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Module {
    pub body: Vec<Ast<ModuleStmt>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ModuleStmt {
    Stmt(Ast<Stmt>)
}
