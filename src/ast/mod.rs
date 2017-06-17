use std::rc::Rc;
use std::ops::Deref;

mod source;

pub use self::source::{SrcFile, SrcPos};

pub mod front;

pub type Ast<T> = Rc<AstNode<T>>;

pub fn create<T>(content_struct: T, file: Rc<SrcFile>, start_byte: usize, end_byte: usize) -> Ast<T> {
    Rc::new(AstNode {content_struct, pos: SrcPos {file, start_byte, end_byte}})
}

 #[derive(Debug)]
pub struct AstNode<T> {
    content_struct: T,
    pub pos: SrcPos,
}

impl<T> AstNode<T> {
    pub fn replace<U>(content_struct: T, prev_ast: Ast<U>) -> Ast<T> {
        Rc::new(AstNode {content_struct, pos: prev_ast.pos.clone()})
    }
}

impl<T> Deref for AstNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.content_struct
    }
}

#[derive(Debug)]
pub struct Ident(pub String);
