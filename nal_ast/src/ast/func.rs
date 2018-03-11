use super::*;

#[derive(Debug)]
pub struct Function {
    pub name: Option<Node<Ident>>,
    pub param: Node<Pattern>,
    pub body: Block<Stmt>,
}
