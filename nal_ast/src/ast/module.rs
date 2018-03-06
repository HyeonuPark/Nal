use prelude::*;

#[derive(Debug)]
pub struct Module {
    pub body: Block<ModuleStmt>,
}

#[derive(Debug)]
pub enum ModuleStmt {
    Stmt(Node<Stmt>),
}
