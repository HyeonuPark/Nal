
extern crate owning_ref;
extern crate nal_ast;

use std::rc::Rc;

mod common;
mod value_ref;
mod env;
mod eval_impl;

pub use common::{Value, Error};
pub use env::Env;

pub fn eval<I, K>(src: &str, globals: I) -> Result<(), Error>
    where K: Into<Rc<str>>, I: IntoIterator<Item=(K, Value)> {
        use owning_ref::RcRef;
        use nal_ast::SourceBuffer;
        use common::{Eval, Control};

        let buf = SourceBuffer::create(src)
                    .map_err(|r| r.to_string())?;
        let program = RcRef::new(buf.into());

        let env = &mut Env::new(program.clone());

        for (k, v) in globals {
            env.decl(k.into(), v);
        }

        program.as_module().eval(env).map_err(|e| match e {
            Control::Return(_) | Control::Break | Control::Continue => {
                unreachable!()
            }
            Control::Error(e) => e,
        })
}
