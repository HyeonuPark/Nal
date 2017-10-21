use ast::module::Module;

mod ctx;
pub use self::ctx::{Ctx, DeclInfo};

mod error;
pub use self::error::Error;

mod check_impl;

#[cfg(test)]
mod tests;

pub fn check(module: &Module) -> Result<(), Vec<Error>> {
    let mut ctx = Ctx::default();

    module.check(&mut ctx);

    let errors = ctx.errors();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub trait Check {
    fn check(&self, ctx: &mut Ctx);
}

impl<T: Check> Check for Option<T> {
    fn check(&self, ctx: &mut Ctx) {
        if let Some(ref inner) = *self {
            inner.check(ctx);
        }
    }
}
