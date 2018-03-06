use prelude::*;

#[derive(Debug)]
pub enum Stmt {
    Expr(Node<Expr>),
    Declare {
        variable: Node<Pattern>,
        init: Node<Expr>,
    },
    If {
        condition: Node<Expr>,
        then: Block<Stmt>,
        otherwise: ElseCase,
    },
    While {
        condition: Node<Expr>,
        body: Block<Stmt>,
    },
}

#[derive(Debug)]
pub enum Pattern {
    Variable(Node<Ident>),
}

#[derive(Debug)]
pub enum ElseCase {
    ElseIf {
        condition: Node<Expr>,
        then: Block<Stmt>,
        otherwise: Box<ElseCase>,
    },
    Else(Block<Stmt>),
    Omit,
}
