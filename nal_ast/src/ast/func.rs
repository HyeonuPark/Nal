use prelude::*;

#[derive(Debug)]
pub struct Function {
    pub name: Option<Node<Ident>>,
    pub parameter: Node<Pattern>,
    pub body: Block<Stmt>,
}
