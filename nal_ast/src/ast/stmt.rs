use super::*;

#[derive(Debug)]
pub enum Stmt {
    Expr(Node<Expr>),
    Declare {
        variable: Node<Pattern>,
        init: Node<Expr>,
    },
    Assign {
        target: Node<Expr>,
        value: Node<Expr>,
    },
    If(IfStmt),
    While {
        condition: Node<Expr>,
        body: Block<Stmt>,
    },
}

#[derive(Debug)]
pub enum Pattern {
    Void,
    Variable(Node<Ident>),
    Tuple(Block<Pattern>),
    Obj(Node<[(Node<Ident>, Node<Pattern>)]>),
}

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Node<Expr>,
    pub then: Block<Stmt>,
    pub else_case: ElseCase,
}

#[derive(Debug)]
pub enum ElseCase {
    ElseIf(Box<IfStmt>),
    Else(Block<Stmt>),
    Omit,
}
