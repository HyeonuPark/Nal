//! Structured control flow representations

use std::collections::HashMap;
use std::fmt;

use nal_ident::Ident;

use crate::ty::Ty;
use crate::instruction::Instr;

pub trait Break: fmt::Debug {
    type Inner: Break;
}

/// A `Step` is the unit of change in program state.
#[derive(Debug)]
pub enum Step<B: Break> {
    /// Execute given instruction using expression stack.
    ///
    /// # Error
    ///
    /// Throw type error if current stack doesn't fulfil this instruction's requirement.
    Instr(Instr<B>),

    /// Execute one of given branches, based on `Enum` type's tag name.
    ///
    /// # Step
    ///
    /// 1. Pop a single value from the stack.
    /// 1. Assume it as `Enum` type.
    /// 1. Separate it into tag and inner value, and push that value back to the stack.
    /// 1. Execute block with matching tag.
    ///
    ///
    /// # Error
    ///
    /// Throw type error if:
    ///
    /// 1. Current stack is empty.
    /// 1. Popped value is not `Enum` type.
    /// 1. Branch block with matching tag not exist.
    /// 1. Any branch produces non-identical program state after execution.
    Branch(HashMap<Ident, Block<B>>),

    /// Execute given block repeatedly, until it breaks out.
    ///
    /// # Step
    ///
    /// 1. Execute given block.
    /// 1. When encounter `Instr(Break(b))` within this block, stop the execution.
    /// 1. If `b` is `CanBreak(Some(b2))`, treat this `Loop` step as `Instr(Break(b2))`.
    /// 1. If execution of given block is done without break, goto step 1 and run it again.
    ///
    /// # Error
    ///
    /// Throw type error if any breakage within given block produces non-identical program state.
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
