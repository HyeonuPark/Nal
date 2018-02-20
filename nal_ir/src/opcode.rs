use common::{Ident, Index, VarName, Value, Constant, Ty};

pub enum Opcode {
    Variable(Variable),
    Record(Record),
    Tuple(Tuple),
    Function(Function),
}

pub enum Variable {
    Localize(Constant, Value),
    Declare(VarName, Ty),
    Get(VarName, Value),
    Set(VarName, Value),
}

pub enum Record {
    Open,
    Push(Ident, Value),
    Spread(Value),
    Close(Value),
    Get {
        parent: Value,
        name: Ident,
        value: Value,
    },
    Set {
        parent: Value,
        name: Ident,
        value: Value,
    },
}

pub enum Tuple {
    Open,
    Push(Value),
    Spread(Value),
    Close(Value),
    Get {
        parent: Value,
        index: Index,
        value: Value,
    },
    Set {
        parent: Value,
        index: Index,
        value: Value,
    },
    Rest {
        parent: Value,
        since: Index,
        value: Value,
    },
}

pub enum Function {
    Open(Value),
    Push(Ident, Value),
    Close(Value),
    Call {
        callee: Value,
        argument: Value,
        result: Value,
    },
}
