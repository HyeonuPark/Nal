use codebuf::{CodeBuf, Span};
use parse_tree::Module as PMod;
use nal_ast::ast::Module as AMod;

mod common;
mod ident;
mod literal;
mod impl_macro;

pub trait Convert<T> {
    fn convert(&self, ctx: &mut Ctx) -> Result<T, ()>;
}

pub struct Ctx<'a> {
    pub errors: Vec<Error>,
    pub buf: &'a CodeBuf
}

pub enum Error {
    InvalidLine(Span),
    UseReservedKeyword(Span),
    InvalidBoolLiteral(Span),
    InvalidNumLiteral(Span),
    UnknownStringEscape(Span),
    InvalidStringChar(Span),
}

pub fn convert(buf: &CodeBuf, module: &PMod) -> Result<AMod, (Option<AMod>, Vec<Error>)> {
    let mut ctx = Ctx {
        errors: Vec::new(),
        buf
    };

    let res = module.convert(&mut ctx);
    let errors = ctx.errors;

    match res {
        Ok(module) => {
            if errors.is_empty() {
                Ok(module)
            } else {
                Err((Some(module), errors))
            }
        }
        Err(_) => Err((None, errors)),
    }
}
