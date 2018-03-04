use common::{Ident, VarName, Value, Ty};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Opcode {
    Variable(Variable),
    Record(Record),
    Tuple(Tuple),
    Function(Function),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Variable {
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
        index: usize,
        value: Value,
    },
    Set {
        parent: Value,
        index: usize,
        value: Value,
    },
    Slice {
        parent: Value,
        skip: usize,
        left: usize,
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
