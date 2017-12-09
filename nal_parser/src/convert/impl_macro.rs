use nal_ast::ast;
use parse_tree as pt;
use super::{Convert, Ctx};

macro_rules! convert {
    ($target:ident($($arg:ident),*)) => (
        impl Convert<ast::$target> for pt::$target {
            fn convert(&self, ctx: &mut Ctx) -> Result<ast::$target, ()> {
                let ($($arg),*) = match *self {
                    pt::$target($(ref $arg),*) => ($($arg.convert(ctx)),*),
                };
                Ok(ast::$target($($arg?),*))
            }
        }
    );
    ($target:ident{$($arg:ident),*}) => (
        impl Convert<ast::$target> for pt::$target {
            fn convert(&self, ctx: &mut Ctx) -> Result<ast::$target, ()> {
                $(let $arg = self.$arg.convert(ctx);)*
                Ok(ast::$target{$($arg: $arg?),*})
            }
        }
    );
    ($target:ident, $($rest:tt)*) => (
        convert!(=collect $target ctx () $($rest)*);
    );

    (=collect $target:ident $ctx:ident ($($prev:tt)*)
        $each:ident,
    $($rest:tt)*) => (
        convert!(=collect $target $ctx ($($prev)*
            pt::$target::$each => ast::$target::$each,
        ) $($rest)*);
    );

    (=collect $target:ident $ctx:ident ($($prev:tt)*)
        $each:ident($($arg:ident),*),
    $($rest:tt)*) => (
        convert!(=collect $target $ctx ($($prev)*
            pt::$target::$each($(ref $arg),*) => {
                $(let $arg = $arg.convert($ctx);)*
                ast::$target::$each($($arg?),*)
            }
        ) $($rest)*);
    );

    (=collect $target:ident $ctx:ident ($($prev:tt)*)
        $each:ident{$($arg:ident),*},
    $($rest:tt)*) => (
        convert!(=collect $target $ctx ($($prev)*
            pt::$target::$each{$(ref $arg),*} => {
                $(let $arg = $arg.convert($ctx);)*
                ast::$target::$each{$($arg: $arg?),*}
            }
        ) $($rest)*);
    );

    (=collect $target:ident $ctx:ident ($($elem:tt)*)) => (
        impl Convert<ast::$target> for pt::$target {
            #[allow(unused_variables)]
            fn convert(&self, $ctx: &mut Ctx) -> Result<ast::$target, ()> {
                Ok(match *self { $($elem)* })
            }
        }
    );
}

convert!(Expr,
    Ident(i),
    Literal(l),
    Tuple(t),
    Obj(b),
    Function(f),
    Call(c, a),
    Prop(p, n),
    Unary(o, e),
    Binary(o, l, r),
    Tagged(t, e),
    Return(e),
    Break,
    Continue,
);

convert!(TupleElem,
    Atom(e),
    Spread(e),
);

convert!(ObjProp,
    Named(n, e),
    Short(n),
    Method(f),
);

convert!(BinaryOp,
    Add, Sub, Mul, Div,
    Eq, Neq, Gt, Gte, Lt, Lte,
    And, Or,
);

convert!(UnaryOp,
    Not, Neg,
);

convert!(Stmt,
    Expr(e),
    If(i),
    While(c, b),
    Function{is_static, func},
    Let(p, e),
    Assign(p, e),
);

convert!(IfStmt(c, b, o));

convert!(IfFalse,
    None,
    Base(b),
    Chain(i),
);

convert!(Pattern,
    Ident{is_mut, ident},
    Tuple(t),
    Obj(o),
);

convert!(TupleElemPattern,
    Atom(p),
    Spread(p),
);

convert!(ObjPropPattern,
    Named(i, p),
    Short(i),
);

convert!(Function{name, params, body});

convert!(FunctionBody,
    Stmt(s),
    Expr(e),
);

convert!(Module{body});

convert!(ModuleStmt,
    Stmt(s),
);
