extern crate internship;

extern crate nal_ir as ir;

pub mod stdnal;

pub mod error;
mod value;
mod scope;
mod proxy;
mod engine;

pub use value::{Value, ValueRef};
pub use engine::Engine;

pub fn exec(src: &ir::EntryModule) -> Result<(), error::RuntimeError> {
    Engine::new()
        //.set_all(stdnal::get_all())
        .exec(src)
}
