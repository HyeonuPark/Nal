extern crate internship;

extern crate nal_ir as ir;

pub mod stdnal;

pub mod error;
pub mod value;
pub mod scope;
pub mod engine;

pub fn exec(_src: &ir::EntryModule) -> Result<(), error::RuntimeError> {
    unimplemented!()
}
