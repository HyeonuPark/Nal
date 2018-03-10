use prelude::*;

#[derive(Debug)]
pub struct Function {
    pub name: Option<Node<Ident>>,
    pub params: Node<Pattern>,
    pub body: Block<Stmt>,
}
