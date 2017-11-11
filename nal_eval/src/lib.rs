extern crate owning_ref;

extern crate nal_ast;

mod common;
pub use common::{Eval, Value, Control, Error, Result};

mod value_ref;
pub use value_ref::{ValueRef, ValueRefMut};

mod env;
pub use env::Env;

mod eval_impl;

pub fn eval(src: &str, env: &Env) -> Result<()> {
    use std::rc::Rc;
    use nal_ast::SourceBuffer;

    let buf = SourceBuffer::create(src, env.names())
                .map_err(|r| r.to_string())?;
    let env = &mut env.child();

    owning_ref::RcRef::new(Rc::new(buf)).eval(env)
}
