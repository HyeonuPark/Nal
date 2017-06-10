use ast::*;

#[derive(Debug)]
pub enum Pattern {
    Ident(Ast<Ident>, Option<Ast<Ty>>),
    MutIdent(Ast<Ident>, Option<Ast<Ty>>),
    Void(Option<Ast<Ty>>),
    Record(Vec<(Ast<Ident>, Ast<Pattern>, bool)>),
    Tuple(Ast<TuplePattern>),
}


#[derive(Debug)]
pub enum TuplePattern {
    Fixed(Vec<Ast<Pattern>>),
    Varadic(Vec<Ast<Pattern>>, Ast<Pattern>, Vec<Ast<Pattern>>),
}
