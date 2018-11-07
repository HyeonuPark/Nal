
use nal_ident::Ident;

use crate::flow::Var;

/// Leaf instructions.
///
/// Instruction modifies program state.
/// It doesn't have operations that can be handled via `Call` instruction
/// like arithmetic operations.
#[derive(Debug)]
pub enum Instr {
    /// > [,] -> !
    ///
    /// Breaks out enclosing loop.
    ///
    /// If its counter is greater than 0, it also breaks nested loop, with given amount.
    /// Breaking out function boundery is same as returning from it.
    ///
    /// # Error
    ///
    /// Throw compile error if its counter is greater than its loop depth.
    Break(usize),
    /// > [,] -> [, Load(var)]
    Push(Var),
    /// > [, a] -> [,], Store(var, a) if Some(var)
    Pop(Option<Var>),
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
