#![allow(dead_code)]

extern crate internship;

pub extern crate nal_ast as ast;
pub extern crate nal_ir as ir;

use std::rc::Rc;
use std::cell::RefCell;

mod scope;
use self::scope::Scope;

#[derive(Debug)]
pub struct ConvertError;

pub type Result<T> = ::std::result::Result<T, ConvertError>;

pub fn convert(input: ast::Module) -> Result<ir::Module> {
    let builder = ir::ModuleBuilder::new();
    let builder = Rc::new(RefCell::new(builder));

    {
        convert_func(
            builder.clone(),
            &mut Scope::new(),
            &ast::Pattern::Void,
            input.body.val().iter().map(|m_stmt| {
                match *m_stmt.val() {
                    ast::ModuleStmt::Stmt(ref stmt) => stmt,
                }
            }),
        )?;
    }

    let builder = Rc::try_unwrap(builder)
        .expect("Some FunctionBuilder are not terminated yet")
        .into_inner();
    Ok(builder.finish())
}

fn convert_func<'a, I: IntoIterator<Item = &'a ast::Node<ast::Stmt>>>(
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
    value: ir::Value
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
                    value: elem,
                });
                declare(scope, builder, pat.val(), elem)?;
            }
        }
        P::Obj(ref obj) => {
            for &(ref ident, ref pat) in obj.val() {
                let elem = builder.value();
                builder.push(ir::Obj::Get {
                    parent: value,
                    name: ir::Ident(ident.val().0.clone()),
                    value: elem,
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
                    let name = ir::Ident(field.val().0.clone());
                    builder.push(ir::Obj::Set {
                        parent,
                        name,
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
                        param: unit_val,
                    },
                    or: ir::Goto {
                        block: on_true,
                        param: unit_val,
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
                    param: unit_val,
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
                            param: unit_val,
                        }));
                    }
                    EC::Omit => {
                        builder.wrap(at_last, ir::ExitCode::Jump(ir::Goto {
                            block: at_last,
                            param: unit_val,
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
                param: unit_val,
            }));

            // check condition
            let cond = convert_expr(scope, builder, condition.val())?;

            builder.wrap(loop_body, ir::ExitCode::Branch {
                when: cond,
                then: ir::Goto {
                    block: loop_body,
                    param: unit_val,
                },
                or: ir::Goto {
                    block: at_last,
                    param: unit_val,
                },
            });

            // while loop body
            scope.child(|scope| {
                for stmt in body.val() {
                    convert_stmt(scope, builder, stmt.val())?;
                }

                Ok(())
            })?;

            builder.wrap(at_last, ir::ExitCode::Jump(ir::Goto {
                block: at_last,
                param: unit_val,
            }));
        }
    }

    Ok(())
}

fn convert_expr(
    _scope: &mut scope::Scope,
    _builder: &mut ir::FunctionBuilder,
    _expr: &ast::Expr,
) -> Result<ir::Value> {
    unimplemented!()
}
