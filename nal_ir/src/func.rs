use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

use common::{Value, BlockToken};
use opcode::Opcode;
use module::ModuleBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub entry: ParamBlock,
    pub blocks: HashMap<BlockToken, ParamBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamBlock {
    pub param: Value,
    pub body: Vec<Opcode>,
    pub exit: ExitCode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExitCode {
    Return(Value),
    Jump(Goto),
    Branch {
        when: Value,
        then: Goto,
        or: Goto,
    },
    Panic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goto {
    pub block: BlockToken,
    pub argument: Value,
}

#[derive(Debug)]
pub struct FunctionBuilder {
    module_builder: Rc<RefCell<ModuleBuilder>>,
    count: usize,
    param: Value,
    current_block: BlockToken,
    current_ops: Vec<Opcode>,
    loop_stack: Vec<(BlockToken, BlockToken)>,
    blocks: HashMap<BlockToken, ParamBlock>,
    entry: BlockToken,
    dead: BlockToken,
}

impl FunctionBuilder {
    pub fn new(module_builder: Rc<RefCell<ModuleBuilder>>) -> Self {
        let mut count = 0;

        let entry = BlockToken::new(&mut count);
        let dead = BlockToken::new(&mut count);
        let param = Value::new(&mut count);

        FunctionBuilder {
            module_builder,
            count,
            param,
            current_block: entry,
            current_ops: vec![],
            loop_stack: vec![],
            blocks: HashMap::new(),
            entry,
            dead,
        }
    }

    pub fn module(&self) -> Ref<ModuleBuilder> {
        self.module_builder.borrow()
    }

    pub fn module_mut(&mut self) -> RefMut<ModuleBuilder> {
        self.module_builder.borrow_mut()
    }

    pub fn module_raw(&self) -> Rc<RefCell<ModuleBuilder>> {
        self.module_builder.clone()
    }

    pub fn unit(&self) -> Value {
        self.module().get_unit().to_value()
    }

    pub fn dead(&self) -> BlockToken {
        self.dead
    }

    pub fn param(&self) -> Value {
        self.param
    }

    pub fn value(&mut self) -> Value {
        Value::new(&mut self.count)
    }

    pub fn block(&mut self) -> BlockToken {
        if self.current_block == self.dead {
            self.dead
        } else {
            BlockToken::new(&mut self.count)
        }
    }

    pub fn push<O: Into<Opcode>>(&mut self, op: O) {
        self.current_ops.push(op.into());
    }

    pub fn wrap(&mut self, next_block: BlockToken, exit: ExitCode) {
        use std::mem::replace;

        let body = replace(&mut self.current_ops, vec![]);

        self.blocks.insert(self.current_block, ParamBlock {
            param: self.param,
            body,
            exit,
        });

        self.current_block = next_block;
        self.param = self.value();
    }

    pub fn loop_push(&mut self, entry: BlockToken, exit: BlockToken) {
        self.loop_stack.push((entry, exit));
    }

    pub fn loop_pop(&mut self, entry: BlockToken, exit: BlockToken) {
        let last_loop = self.loop_stack.pop()
            .expect("loop_pop must be matched with loop_push");
        assert_eq!(
            last_loop, (entry, exit),
            "loop_pop must be matched with loop_push of same block",
        );
    }

    pub fn current_loop(&self) -> (BlockToken, BlockToken) {
        *self.loop_stack.last()
            .expect("current_loop should be positioned after some loop_push")
    }

    pub fn finish(mut self) -> Function {
        assert!(
            self.current_ops.is_empty(),
            "FunctionBuilder failed to finish: this builder is incomplete\n{:#?}",
            self,
        );

        let entry = self.blocks.remove(&self.entry).unwrap();
        self.blocks.remove(&self.dead);

        Function {
            entry,
            blocks: self.blocks,
        }
    }
}
