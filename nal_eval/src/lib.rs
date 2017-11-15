extern crate owning_ref;
extern crate nal_ast;

mod common;
mod value_ref;
mod env;
mod eval_impl;

pub use common::{Value, Error};
pub use env::Env;

pub fn eval(src: &str, env: &Env) -> Result<(), Error> {
    use std::rc::Rc;
    use nal_ast::SourceBuffer;
    use common::{Eval, Control};

    let buf = SourceBuffer::create(src, env.names())
                .map_err(|r| r.to_string())?;
    let env = &mut env.child();

    owning_ref::RcRef::new(Rc::new(buf)).eval(env).map_err(|e| match e {
        Control::Return(_) | Control::Break | Control::Continue => {
            unreachable!()
        }
        Control::Error(e) => e,
    })
}
