extern crate nalc_parser as parser;
extern crate nalc_atoi as atoi;
extern crate nali;

pub mod error;

pub fn exec(src: &str) -> Result<(), error::Error> {
    let ast = parser::parse(src)?;
    let ir = atoi::convert(&ast)?;
    nali::exec(&ir)?;

    Ok(())
}
