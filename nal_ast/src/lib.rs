#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate nom_locate;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[cfg(test)]
extern crate serde_yaml;

pub mod ast;

mod parse;
mod check;

mod buffer;
pub use buffer::SourceBuffer;

mod report;
pub use report::Report;
