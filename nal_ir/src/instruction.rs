
use nal_symbol::Symbol;

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
    /// > [, .name field, ...] -> [, {name = field, ...}]
    ///
    /// Create new record of given size from stack values.
    /// Each values popped from the stack should be enum type,
    /// and its tag will be its field name.
    ///
    /// # Error
    ///
    /// Throw compile error if any of popped value is not an enum type,
    /// or 2 of them has same tag name.
    RecordNew(usize),
    /// > [, a] -> [, a.ident]
    ///
    /// # Error
    ///
    /// Throw compile error if the record doesn't have field `ident`.
    RecordGet(Symbol),
    /// > [, a, b] -> [, a[ident = b]]
    ///
    /// # Error
    ///
    /// Throw compile error if the record `a` doesn't have field `ident`.
    RecordSet(Symbol),
    /// > [, {name = field, ...}] -> [, .name field, ...]
    ///
    /// Spreads record's fields to the stack.
    /// Actual order of spreaded fields are not specified.
    /// Each fields are pushed to the stack as an enum type,
    /// whose tag name is same as its field name.
    RecordSpread,
    /// > [, ..a] -> [, (..a)], Len(a) == count
    TupleNew(usize),
    /// > [, a] -> [, a.idx]
    ///
    /// # Error
    ///
    /// Throw comiple error on index out of bound.
    TupleGet(usize),
    /// > [, a, b] -> [, a[idx = b]]
    ///
    /// # Error
    ///
    /// Throw comiple error on index out of bound.
    TupleSet(usize),
    /// > [, (..a)] -> [, ..a]
    TupleSpread,
    /// > [, a] -> [, .ident a]
    Enum(Symbol),
}

#[derive(Debug, Clone, Copy)]
pub enum RefType {
    Imut,
    Mut,
    Move,
}
