use super::{Node, Block, Stmt};

#[derive(Debug, PartialEq, Clone)]
pub struct Module {
    pub body: Block<ModuleStmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ModuleStmt {
    Stmt(Node<Stmt>),
}
