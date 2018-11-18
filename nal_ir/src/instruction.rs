
use nal_ident::Ident;

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
    /// [, ...a] -> [,], Len(a) == count
    Pop(usize),
    /// > [,] -> [, Global]
    ///
    /// Push a record of global values to the stack.
    Global,
    /// > [, a, ...b] -> [, a, ...b, Reference(a, ref_type)], Len(b) == count
    ///
    /// Push a reference of the value at stack's given depth.
    Ref(RefType, usize),
    /// > [, a, b] -> [, Call(a, b)]
    ///
    /// Pop two values from the stack,
    /// call the lower value with upper value as an argument.
    Call,
    /// > [,] -> [, {}]
    RecordNew,
    /// > [, a] -> [, a.ident]
    RecordGet(Ident),
    /// > [, a, b] -> [, a[ident = b]]
    RecordSet(Ident),
    /// > [, ..a] -> [, (..a)], Len(a) == count
    TupleNew(usize),
    /// > [, a] -> [, a.idx]
    TupleGet(usize),
    /// > [, a, b] -> [, a[idx = b]]
    TupleSet(usize),
    /// > [, a] -> [, .ident a]
    Enum(Ident),
}

#[derive(Debug, Clone, Copy)]
pub enum RefType {
    Imut,
    Mut,
    Move,
}
