//! Structured control flow representations.
//!
//! Types here are designed mainly for type check and inference.

use nal_symbol::Symbol;

use crate::instruction::Instr;

pub type Block = Vec<Step>;

/// A `Step` is the unit of change in program state.
#[derive(Debug)]
pub enum Step {
    /// Execute given instruction using expression stack.
    ///
    /// # Error
    ///
    /// Throw compile error if current stack doesn't fulfil this instruction's requirement.
    Instr(Instr),

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
    /// Throw compile error if any of the following conditons are not met.
    ///
    /// 1. Current stack has a value with `Enum` at its top.
    /// 1. This step contains branch with matching tag.
    /// 1. All branches should produce same program state in the end.
    Branch(Vec<(Symbol, Block)>),

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
    /// Throw compile error if any of the following conditions are not met.
    ///
    /// 1. If execution reaches the end of the given block, the program state should be same
    ///     as the state before this `Loop` step.
    /// 1. All breaks in this block should produce same program state.
    ///
    /// Throw compile error if any breakage within given block produces non-identical program state.
    Loop(Block),
}

#[derive(Debug)]
pub struct Func {
    // `Break` in function body means returning function.
    body: Block,
}
