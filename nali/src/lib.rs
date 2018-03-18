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
pub use error::Error;

pub fn exec(src: &ir::EntryModule) -> Result<(), Error> {
    Engine::new()
        //.set_all(stdnal::get_all())
        .exec(src)
}
