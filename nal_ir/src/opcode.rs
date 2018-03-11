use common::{Ident, VarName, Value, Ty};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Opcode {
    Variable(Variable),
    Obj(Obj),
    Tuple(Tuple),
    Exec(Exec),
}

impl From<Variable> for Opcode {
    fn from(value: Variable) -> Self {
        Opcode::Variable(value)
    }
}

impl From<Obj> for Opcode {
    fn from(value: Obj) -> Self {
        Opcode::Obj(value)
    }
}

impl From<Tuple> for Opcode {
    fn from(value: Tuple) -> Self {
        Opcode::Tuple(value)
    }
}

impl From<Exec> for Opcode {
    fn from(value: Exec) -> Self {
        Opcode::Exec(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Variable {
    Declare(VarName, Ty),
    Get(VarName, Value),
    Set(VarName, Value),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Obj {
    Open,
    Push(Ident, Value),
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
pub enum Exec {
    Call {
        callee: Value,
        argument: Value,
        result: Value,
    },
    LogicNot {
        operand: Value,
        result: Value,
    },
}
