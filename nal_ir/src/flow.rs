//! Structured control flow representations

use std::collections::HashMap;
use std::fmt;

use nal_ident::Ident;

use crate::ty::Ty;
use crate::instruction::Instr;

pub trait Break: fmt::Debug {
    type Inner: Break;
}

#[derive(Debug)]
pub enum Step<B: Break> {
    Instr(Instr<B>),
    Branch(HashMap<Ident, Block<B>>),
    Loop(Block<CanBreak<B>>),
}

#[derive(Debug)]
pub struct Block<B: Break> {
    depth: usize,
    scope: HashMap<Slot, Ty>,
    body: Vec<Step<B>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slot {
    name: Ident,
    order: usize,
    depth: usize,
}

#[derive(Debug, Default)]
pub struct CanBreak<B: Break>(Option<B>);

impl Break for () {
    type Inner = ();
}

impl<B: Break> Break for CanBreak<B> {
    type Inner = B;
}
