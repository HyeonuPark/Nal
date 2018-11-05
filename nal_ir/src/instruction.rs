
use nal_ident::Ident;

use crate::flow::{Break, Slot};

/// Leaf instructions.
///
/// Instruction modifies program state.
/// It doesn't have operations that can be handled via `Call` instruction
/// like arithmetic operations.
#[derive(Debug)]
pub enum Instr<B: Break> {
    /// > [,] -> !
    ///
    /// Breaks enclosing loop with inner type.
    /// If `inner` is `CanBreak(Some(b))`,
    /// break will be bubbled up to the outer loop with `b`.
    Break(B::Inner),
    /// > [,] -> [, Load(slot)]
    Push(Slot),
    /// > [, a] -> [,], Store(slot, a) if Some(slot)
    Pop(Option<Slot>),
    /// > [, a, ..b] -> [, a, ..b, a], Len(b) == count
    Dupe(usize),
    /// > [, a, b] -> [, Call(a, b)]
    Call,
    /// > [,] -> [, {}]
    Record,
    /// > [, a] -> [, a.ident]
    FieldGet(Ident),
    /// > [, a, b] -> [, a[ident = b]]
    FieldSet(Ident),
    /// > [, ..a] -> [, (..a)], Len(a) == count
    Tuple(usize),
    /// > [, a] -> [, a.idx]
    IndexGet(usize),
    /// > [, a, b] -> [, a[idx = b]]
    IndexSet(usize),
    /// > [, a] -> [, .ident a]
    Enum(Ident),
}
