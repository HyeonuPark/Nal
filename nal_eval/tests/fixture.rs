#[macro_use]
extern crate pretty_assertions;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

extern crate nal_ast;
extern crate nal_eval;

use std::rc::Rc;
use std::cell::RefCell;

use serde_yaml::from_str as yaml;

use nal_eval::{eval, Value, Env};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum SValue {
    Unit,
    Num(f64),
    Bool(bool),
}

use self::Value as V;
use self::SValue as S;

impl From<Value> for SValue {
    fn from(v: Value) -> Self {
        match v {
            V::Unit => S::Unit,
            V::Num(v) => S::Num(v),
            V::Bool(v) => S::Bool(v),
            _ => panic!("Can't serialize function values"),
        }
    }
}

fn eval_print(src: &str) -> Vec<SValue> {
    let mut env = Env::default();
    let content = Rc::new(RefCell::new(Vec::new()));
    let content2 = Rc::clone(&content);

    let print = Value::Native(Rc::new(move|args| {
        content2.borrow_mut().extend(args);
        Ok(Value::Unit)
    }));

    env.decl("print".into(), print);

    match eval(src, &env) {
        Err(e) => panic!("Failed to eval: {:?}", e),
        Ok(_) => {
            content.borrow().iter()
                .map(|v| v.clone().into())
                .collect()
        }
    }
}

macro_rules! fixtures {
    ($($name:ident, $test:expr)*) => ($(
        #[test]
        fn $name() {
            assert_eq!(
                eval_print(include_str!(concat!("fixtures/", $test, ".nal"))),
                yaml::<Vec<S>>(include_str!(concat!("fixtures/", $test, ".yml")))
                    .expect(concat!("Failed to parse ", $test, ".yml")),
                concat!("\n\nFailed: ", $test, " - nal != yml\n\n")
            );
        }
    )*);
}

fixtures!(
    simple, "simple"
);
