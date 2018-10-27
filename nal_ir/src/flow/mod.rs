//! Structured control flow representations

use std::collections::HashMap;

use nal_ident::Ident;

use super::ty::Ty;

mod private {
    use std::fmt;

    pub trait Break: fmt::Debug {
        type Inner: Break;
    }
}

use self::private::Break;

#[derive(Debug)]
pub enum Step<B: Break> {
    Instr(Instr<B>),
    IfElse(Vec<Step<B>>, Vec<Step<B>>),
    Block(Block<CanBreak<B>>),
    Loop(Block<CanBreak<B>>),
}

#[derive(Debug)]
pub struct Block<B: Break> {
    depth: usize,
    scope: HashMap<Ref, Ty>,
    body: Vec<Step<B>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ref {
    name: Ident,
    order: usize,
    depth: usize,
}

#[derive(Debug, Default)]
pub struct CanBreak<B: Break>(B);

impl Break for () {
    type Inner = ();
}

impl<B: Break> Break for CanBreak<B> {
    type Inner = B;
}

#[derive(Debug)]
pub enum Instr<B: Break> {
    Break(B::Inner),
    Push(Ref),
    Pop(Option<Ref>),
    Call,
}
