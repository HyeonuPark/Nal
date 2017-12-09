use codebuf::Node;
use nal_ast::ast;
use parse_tree as pt;
use super::{Convert, Ctx, Error as E};

impl Convert<bool> for bool {
    fn convert(&self, _ctx: &mut Ctx) -> Result<bool, ()> {
        Ok(*self)
    }
}

impl<T: Convert<U>, U> Convert<Box<U>> for Box<T> {
    fn convert(&self, ctx: &mut Ctx) -> Result<Box<U>, ()> {
        Ok(Box::new((&**self).convert(ctx)?))
    }
}

impl<T: Convert<U>, U> Convert<Option<U>> for Option<T> {
    fn convert(&self, ctx: &mut Ctx) -> Result<Option<U>, ()> {
        match *self {
            None => Ok(None),
            Some(ref t) => t.convert(ctx).map(Some),
        }
    }
}

impl<T: Convert<U>, U> Convert<Node<U>> for Node<T> {
    fn convert(&self, ctx: &mut Ctx) -> Result<Node<U>, ()> {
        (&**self).convert(ctx).map(|u| Node::new(self.span, u))
    }
}

impl<T: Convert<U>, U> Convert<ast::Block<U>> for pt::Block<T> {
    fn convert(&self, ctx: &mut Ctx) -> Result<ast::Block<U>, ()> {
        Ok(Node::new(
            self.span,
            self.iter()
                .flat_map(|res| match *res {
                    Err(ref node) => {
                        ctx.errors.push(E::InvalidLine(node.span));
                        None
                    }
                    Ok(ref t) => t.convert(ctx).ok()
                })
                .collect::<Vec<_>>()
        ))
    }
}
