use common::{Ident, VarName, Slot, Ty, FuncToken};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Opcode {
    Variable(Variable),
    Obj(Obj),
    Tuple(Tuple),
    Misc(Misc),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Variable {
    Declare(VarName, Ty),
    Get(VarName, Slot),
    Set(VarName, Slot),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Obj {
    Open,
    Push(Ident, Slot),
    Close(Slot),
    Get {
        parent: Slot,
        name: Ident,
        result: Slot,
    },
    Set {
        parent: Slot,
        name: Ident,
        value: Slot,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Tuple {
    Open,
    Push(Slot),
    Close(Slot),
    Get {
        parent: Slot,
        index: usize,
        result: Slot,
    },
    Set {
        parent: Slot,
        index: usize,
        value: Slot,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Misc {
    Call {
        callee: Slot,
        argument: Slot,
        result: Slot,
    },
    LogicNot {
        operand: Slot,
        result: Slot,
    },
    Closure {
        name: Option<VarName>,
        function: FuncToken,
        result: Slot,
    }
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

impl From<Misc> for Opcode {
    fn from(value: Misc) -> Self {
        Opcode::Misc(value)
    }
}
