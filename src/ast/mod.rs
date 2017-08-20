use std::rc::Rc;
use std::ops::Deref;

mod source;

pub use self::source::{Src, SrcPos};

pub mod front;

pub type Ast<T> = Rc<AstNode<T>>;

 #[derive(Debug)]
pub struct AstNode<T> {
    content_struct: T,
    pub pos: SrcPos,
}

pub fn create<T>(ast_data: T, file: Src, start_byte: usize, end_byte: usize) -> Ast<T> {
    Rc::new(AstNode {
        content_struct: ast_data, 
        pos: SrcPos {
            file, 
            start_byte, 
            end_byte
        }
    })
}

pub fn replace<T, U>(prev_ast: Ast<T>, new_data: U) -> Ast<U> {
    Rc::new(AstNode {
        content_struct: new_data,
        pos: prev_ast.pos.clone(),
    })
}

impl<T> Deref for AstNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.content_struct
    }
}

#[derive(Debug)]
pub struct Ident(pub String);
