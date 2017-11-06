use nal_ast::ast::prelude::{Pattern};

use common::{Env, Value, Result};

use self::Pattern::*;

pub fn decl_pattern(env: &mut Env, pat: &Pattern, init: Value) -> Result<()> {
    match *pat {
        Ident(ref name, is_mut) => {
            if is_mut {
                env.decl_mut(name, init);
            } else {
                env.decl(name, init);
            }
        }
        Obj(ref elems) => {
            match init {
                Value::Obj(table) => {
                    for &(ref name, ref subpat) in elems {
                        let prop = table[name as &str].clone();
                        decl_pattern(env, subpat, prop)?;
                    }
                }
                _ => Err("Primitive type properties are not implemented")?,
            }
        }
    }

    Ok(())
}

pub fn assign_pattern(env: &mut Env, pat: &Pattern, init: Value) -> Result<()> {
    match *pat {
        Ident(ref name, _) => {
            env.assign(name, init)?;
        }
        Obj(ref elems) => {
            match init {
                Value::Obj(table) => {
                    for &(ref name, ref subpat) in elems {
                        let prop = table[name as &str].clone();
                        assign_pattern(env, subpat, prop)?;
                    }
                }
                _ => Err("Primitive type properties are not implemented")?,
            }
        }
    }

    Ok(())
}
