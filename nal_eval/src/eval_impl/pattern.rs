use nal_ast::ast::prelude::Pattern as P;

use common::prelude::*;

pub fn decl_pattern(env: &mut Env, pat: &P, init: Value) -> Result<()> {
    match *pat {
        P::Ident(ref ident, is_mut) => {
            if is_mut {
                env.decl_mut(ident.name(), init);
            } else {
                env.decl(ident.name(), init);
            }
        }
        P::Obj(ref elems) => {
            match init {
                Value::Obj(table) => {
                    for &(ref name, ref subpat) in elems {
                        let prop = match table.get(name as &str) {
                            Some(prop) => prop.clone(),
                            None => Err(format!("Invalid structure - \
                                prop {} not exist", name as &str))?,
                        };

                        decl_pattern(env, subpat, prop)?;
                    }
                }
                _ => Err("Property of primitive values are not implemented")?,
            }
        }
    }

    Ok(())
}

pub fn assign_pattern(env: &mut Env, pat: &P, init: Value) -> Result<()> {
    match *pat {
        P::Ident(ref name, _) => {
            env.get_mut(name)?.set(init);
        }
        P::Obj(ref elems) => {
            match init {
                Value::Obj(mut table) => {
                    for &(ref name, ref subpat) in elems {
                        let prop = match table.remove(name as &str) {
                            Some(prop) => prop,
                            None => Err(format!("Invalid structure - \
                                prop {} not exist", name as &str))?,
                        };

                        assign_pattern(env, subpat, prop)?;
                    }
                }
                _ => Err("Property of primitive values are not implemented")?,
            }
        }
    }

    Ok(())
}
