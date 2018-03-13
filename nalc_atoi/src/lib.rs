extern crate internship;

pub extern crate nal_ast as ast;
pub extern crate nal_ir as ir;

use std::rc::Rc;
use std::cell::RefCell;

use ir::Slot;

mod scope;
use self::scope::Scope;

#[derive(Debug)]
pub struct ConvertError;

pub type Result<T> = ::std::result::Result<T, ConvertError>;

pub trait Convert {
    type Output;

    fn convert(&self) -> Result<Self::Output>;
}

impl Convert for ast::Module {
    type Output = ir::EntryModule;

    fn convert(&self) -> Result<ir::EntryModule> {
        let builder = ir::ModuleBuilder::new();
        let builder = Rc::new(RefCell::new(builder));

        let main = convert_func(
            builder.clone(),
            &mut Scope::new(),
            &ast::Pattern::Void,
            self.body.val().iter().map(|m_stmt| {
                match *m_stmt.val() {
                    ast::ModuleStmt::Stmt(ref stmt) => stmt,
                }
            }),
        )?;

        let builder = Rc::try_unwrap(builder)
            .expect("Some FunctionBuilders are not terminated yet")
            .into_inner();
        let module = builder.finish();

        Ok(ir::EntryModule {
            module,
            main,
        })
    }
}

impl Convert for ast::Ident {
    type Output = ir::Ident;

    fn convert(&self) -> Result<ir::Ident> {
        Ok(ir::Ident(self.0.clone()))
    }
}

fn convert_func<'a, I: Iterator<Item = &'a ast::Node<ast::Stmt>>>(
    module: Rc<RefCell<ir::ModuleBuilder>>,
    scope: &mut scope::Scope,
    arg: &ast::Pattern,
    body: I,
) -> Result<ir::Function> {
    let mut builder = ir::FunctionBuilder::new(module);

    {
        let builder = &mut builder;

        let param = builder.param();
        declare(scope, builder, arg, param)?;

        for stmt in body {
            convert_stmt(scope, builder, stmt.val())?;
        }
    }

    Ok(builder.finish())
}

fn declare(
    scope: &mut Scope,
    builder: &mut ir::FunctionBuilder,
    pat: &ast::Pattern,
    value: Slot
) -> Result<()> {
    use ast::Pattern as P;

    match *pat {
        P::Void => {}
        P::Variable(ref ident) => {
            let name = scope.declare(ident.val());
            builder.push(ir::Variable::Declare(name.clone(), ir::Ty::default()));
            builder.push(ir::Variable::Set(name, value));
        }
        P::Tuple(ref tuple) => {
            for (idx, pat) in tuple.val().iter().enumerate() {
                let elem = builder.value();
                builder.push(ir::Tuple::Get {
                    parent: value,
                    index: idx,
                    result: elem,
                });
                declare(scope, builder, pat.val(), elem)?;
            }
        }
        P::Obj(ref obj) => {
            for &(ref ident, ref pat) in obj.val() {
                let elem = builder.value();
                builder.push(ir::Obj::Get {
                    parent: value,
                    name: ident.val().convert()?,
                    result: elem,
                });
                declare(scope, builder, pat.val(), elem)?;
            }
        }
    }

    Ok(())
}

fn convert_stmt(
    scope: &mut scope::Scope,
    builder: &mut ir::FunctionBuilder,
    stmt: &ast::Stmt,
) -> Result<()> {
    use ast::Stmt as S;
    use ast::Expr as X;

    let unit_val = builder.unit();

    match *stmt {
        S::Expr(ref expr) => {
            convert_expr(scope, builder, expr.val())?;
        }
        S::Declare { ref variable, ref init } => {
            let value = convert_expr(scope, builder, init.val())?;
            declare(scope, builder, variable.val(), value)?;
        }
        S::Assign { ref target, ref value } => {
            let value = convert_expr(scope, builder, value.val())?;

            match *target.val() {
                X::Variable(ref ident) => {
                    let name = scope.get(ident.val())?;
                    builder.push(ir::Variable::Set(name, value));
                }
                X::TupleField { ref parent, ref field } => {
                    let parent = convert_expr(scope, builder, parent.val())?;
                    let index = *field.val();
                    builder.push(ir::Tuple::Set {
                        parent,
                        index,
                        value
                    });
                }
                X::ObjField { ref parent, ref field } => {
                    let parent = convert_expr(scope, builder, parent.val())?;
                    builder.push(ir::Obj::Set {
                        parent,
                        name: field.val().convert()?,
                        value,
                    });
                }
                _ => return Err(ConvertError),
            }
        }
        S::If(ref if_stmt) => {
            let at_last = builder.block();
            handle_if(scope, builder, if_stmt, at_last)?;

            fn handle_if(
                scope: &mut Scope,
                builder: &mut ir::FunctionBuilder,
                if_stmt: &ast::IfStmt,
                at_last: ir::BlockToken,
            ) -> Result<()> {
                let unit_val = builder.unit();
                let on_true = builder.block();
                let on_false = builder.block();

                let cond = convert_expr(scope, builder, if_stmt.condition.val())?;
                builder.wrap(on_true, ir::ExitCode::Branch {
                    when: cond,
                    then: ir::Goto {
                        block: on_true,
                        argument: unit_val,
                    },
                    or: ir::Goto {
                        block: on_true,
                        argument: unit_val,
                    },
                });

                // condition is true
                scope.child(|scope| {
                    for stmt in if_stmt.then.val() {
                        convert_stmt(scope, builder, stmt.val())?;
                    }

                    Ok(())
                })?;

                builder.wrap(on_false, ir::ExitCode::Jump(ir::Goto {
                    block: on_false,
                    argument: unit_val,
                }));

                // condition is false
                use ast::ElseCase as EC;

                match if_stmt.else_case {
                    EC::ElseIf(ref if_stmt) => handle_if(
                        scope, builder, if_stmt, at_last
                    )?,
                    EC::Else(ref block) => {
                        scope.child(|scope| {
                            for stmt in block.val() {
                                convert_stmt(scope, builder, stmt.val())?;
                            }

                            Ok(())
                        })?;

                        builder.wrap(at_last, ir::ExitCode::Jump(ir::Goto {
                            block: at_last,
                            argument: unit_val,
                        }));
                    }
                    EC::Omit => {
                        builder.wrap(at_last, ir::ExitCode::Jump(ir::Goto {
                            block: at_last,
                            argument: unit_val,
                        }));
                    }
                }

                Ok(())
            }
        }
        S::While { ref condition, ref body } => {
            let check = builder.block();
            let loop_body = builder.block();
            let at_last = builder.block();

            builder.wrap(check, ir::ExitCode::Jump(ir::Goto {
                block: check,
                argument: unit_val,
            }));

            // check condition
            let cond = convert_expr(scope, builder, condition.val())?;

            builder.wrap(loop_body, ir::ExitCode::Branch {
                when: cond,
                then: ir::Goto {
                    block: loop_body,
                    argument: unit_val,
                },
                or: ir::Goto {
                    block: at_last,
                    argument: unit_val,
                },
            });

            // while loop body

            builder.loop_push(check, at_last);

            scope.child(|scope| {
                for stmt in body.val() {
                    convert_stmt(scope, builder, stmt.val())?;
                }

                Ok(())
            })?;

            builder.wrap(at_last, ir::ExitCode::Jump(ir::Goto {
                block: check,
                argument: unit_val,
            }));

            builder.loop_pop(check, at_last);
        }
    }

    Ok(())
}

fn convert_expr(
    scope: &mut scope::Scope,
    builder: &mut ir::FunctionBuilder,
    expr: &ast::Expr,
) -> Result<Slot> {
    use ast::{Expr as X, Literal as Lit};

    let unit_val = builder.unit();

    Ok(match *expr {
        X::Variable(ref ident) => {
            match scope.get(ident.val()) {
                Ok(name) => {
                    let value = builder.value();
                    builder.push(ir::Variable::Get(name, value));
                    value
                }
                Err(_) => {
                    builder.module_mut()
                        .add_free_var(ident.val().convert()?)
                        .to_value()
                }
            }
        }
        X::Literal(ref lit) => {
            match *lit.val() {
                Lit::Bool(value) => builder.module().get_bool(value),
                Lit::Num(value) => builder.module_mut().add_num(value),
                Lit::Str(ref value) => builder.module_mut().add_str(value.to_string()),
            }.to_value()
        }
        X::Tuple(ref elems) => {
            builder.push(ir::Tuple::Open);

            for elem in elems.val() {
                match *elem.val() {
                    ast::TupleElem::Atom(ref expr) => {
                        let value = convert_expr(scope, builder, expr.val())?;
                        builder.push(ir::Tuple::Push(value));
                    }
                }
            }

            let value = builder.value();
            builder.push(ir::Tuple::Close(value));
            value
        }
        X::Obj(ref elems) => {
            builder.push(ir::Obj::Open);

            for elem in elems.val() {
                match *elem.val() {
                    ast::ObjElem::Named(ref ident, ref expr) => {
                        let value = convert_expr(scope, builder, expr.val())?;
                        builder.push(ir::Obj::Push(ident.val().convert()?, value));
                    }
                }
            }

            let value = builder.value();
            builder.push(ir::Obj::Close(value));
            value
        }
        X::Function(ref func) => {
            let func = func.val();
            // let func_const = builder.module_mut().get_const();

            scope.child(|scope| {
                let name = func.name.as_ref().map(|ident| scope.declare(ident.val()));

                let func = convert_func(
                    builder.module_raw(),
                    scope, func.param.val(),
                    func.body.val().iter(),
                )?;

                let result = builder.value();
                let token = builder.module_mut().add_func(func);
                builder.push(ir::Misc::Closure {
                    name,
                    function: token,
                    result,
                });

                Ok(result)
            })?
        }
        X::Unary(op, ref expr) => {
            use ast::UnaryOp as O;

            let inner = convert_expr(scope, builder, expr.val())?;

            match op {
                O::Not => {
                    let value = builder.value();
                    builder.push(ir::Misc::LogicNot {
                        operand: inner,
                        result: value,
                    });

                    value
                }
                O::Neg => method_call(builder, inner, "negate", unit_val),
            }
        }
        X::Binary(op, ref left, ref right) => {
            use ast::BinaryOp as O;

            let bool_true = builder.module().get_bool(true).to_value();

            macro_rules! method {
                ($name:expr) => (
                    {
                        let left = convert_expr(scope, builder, left.val())?;
                        let right = convert_expr(scope, builder, right.val())?;
                        method_call(builder, left, $name, right)
                    }
                );
            }

            match op {
                O::And => {
                    let eval_right = builder.block();
                    let at_last = builder.block();

                    // first evaluate left
                    let left = convert_expr(scope, builder, left.val())?;
                    builder.wrap(eval_right, ir::ExitCode::Branch {
                        when: left,
                        then: ir::Goto {
                            block: eval_right,
                            argument: unit_val,
                        },
                        or: ir::Goto {
                            block: at_last,
                            argument: bool_true,
                        },
                    });

                    // if true, evaluate right
                    let right = convert_expr(scope, builder, right.val())?;
                    builder.wrap(at_last, ir::ExitCode::Jump(ir::Goto {
                        block: at_last,
                        argument: right,
                    }));

                    // and return received parameter
                    builder.param()
                }

                O::Or => {
                    let eval_right = builder.block();
                    let at_last = builder.block();

                    // first, evaluate left
                    let left = convert_expr(scope, builder, left.val())?;
                    let not_left = builder.value();
                    builder.push(ir::Misc::LogicNot {
                        operand: left,
                        result: not_left,
                    });
                    builder.wrap(eval_right, ir::ExitCode::Branch {
                        when: not_left,
                        then: ir::Goto {
                            block: eval_right,
                            argument: unit_val,
                        },
                        or: ir::Goto {
                            block: at_last,
                            argument: bool_true,
                        },
                    });

                    // if false, evaluate right
                    let right = convert_expr(scope, builder, right.val())?;
                    builder.wrap(at_last, ir::ExitCode::Jump(ir::Goto {
                        block: at_last,
                        argument: right,
                    }));

                    // and return received parameter
                    builder.param()
                }
                O::Add => method!("add"),
                O::Sub => method!("subtract"),
                O::Mul => method!("multiply"),
                O::Div => method!("divide"),
                O::Eq  => method!("equals"),
                O::Neq => method!("not_equals"),
                O::Gt  => method!("greater_than"),
                O::Gte => method!("not_less_than"),
                O::Lt  => method!("less_than"),
                O::Lte => method!("not_greater_than"),
            }
        }
        X::Call { ref callee, ref argument } => {
            let callee = convert_expr(scope, builder, callee.val())?;
            let argument = convert_expr(scope, builder, argument.val())?;
            let result = builder.value();
            builder.push(ir::Misc::Call {
                callee,
                argument,
                result,
            });
            result
        }
        X::ObjField { ref parent, ref field } => {
            let parent = convert_expr(scope, builder, parent.val())?;
            let result = builder.value();
            builder.push(ir::Obj::Get {
                parent,
                name: field.val().convert()?,
                result,
            });
            result
        }
        X::TupleField { ref parent, ref field } => {
            let parent = convert_expr(scope, builder, parent.val())?;
            let result = builder.value();
            builder.push(ir::Tuple::Get {
                parent,
                index: *field.val(),
                result,
            });
            result
        }
        X::Return(ref value) => {
            let value = match *value {
                Some(ref value) => convert_expr(scope, builder, value.val())?,
                None => unit_val,
            };
            let dead_block = builder.dead();
            builder.wrap(dead_block, ir::ExitCode::Return(value));
            unit_val
        }
        X::Break => {
            let dead_block = builder.dead();
            let loop_exit = builder.current_loop().1;

            builder.wrap(dead_block, ir::ExitCode::Jump(ir::Goto {
                block: loop_exit,
                argument: unit_val,
            }));
            unit_val
        }
        X::Continue => {
            let dead_block = builder.dead();
            let loop_entry = builder.current_loop().0;

            builder.wrap(dead_block, ir::ExitCode::Jump(ir::Goto {
                block: loop_entry,
                argument: unit_val,
            }));
            unit_val
        }
    })
}

fn method_call(
    builder: &mut ir::FunctionBuilder,
    parent: Slot,
    name: &str,
    argument: Slot
) -> Slot {
    let method = builder.value();
    let result = builder.value();

    builder.push(ir::Obj::Get {
        parent: parent,
        name: ir::Ident(name.into()),
        result: method,
    });
    builder.push(ir::Misc::Call {
        callee: method,
        argument,
        result,
    });

    result
}
