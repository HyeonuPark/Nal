use super::{Span, Block, Stmt};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Module {
    pub body: Block<ModuleStmt>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ModuleStmt {
    Stmt(Span<Stmt>),
}
