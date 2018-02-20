use common::{Ident, Index, VarName, Value, ConstToken, Ty};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Opcode {
    Variable(Variable),
    Record(Record),
    Tuple(Tuple),
    Function(Function),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Variable {
    Localize(ConstToken, Value),
    Declare(VarName, Ty),
    Get(VarName, Value),
    Set(VarName, Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Function {
    Call {
        callee: Value,
        argument: Value,
        result: Value,
    },
}
