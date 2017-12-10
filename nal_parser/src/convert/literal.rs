use codebuf::{Node, Span};
use nal_ast::ast;
use parse_tree as pt;
use super::{Convert, Ctx, Error as E};

impl Convert<Node<ast::Literal>> for Node<pt::Literal> {
    fn convert(&self, ctx: &mut Ctx) -> Result<Node<ast::Literal>, ()> {
        let span = self.span;

        Ok(Node::new(
            span,
            match **self {
                pt::Literal::Bool => convert_bool(span, ctx)?,
                pt::Literal::Num => convert_num(span, ctx)?,
                pt::Literal::Str => convert_str(span, ctx)?,
            }
        ))
    }
}

fn convert_bool(span: Span, ctx: &mut Ctx) -> Result<ast::Literal, ()> {
    let parsed = ctx.buf.span(span).parse::<bool>();

    match parsed {
        Ok(v) => Ok(ast::Literal::Bool(v)),
        Err(_) => {
            ctx.errors.push(E::InvalidBoolLiteral(span));
            Err(())
        }
    }
}

fn convert_num(span: Span, ctx: &mut Ctx) -> Result<ast::Literal, ()> {
    let escaped: String = ctx.buf.span(span).chars()
        .filter(|&c| !c.is_whitespace() && c != '_')
        .collect();
    let parsed = escaped.parse::<f64>();

    match parsed {
        Ok(v) => Ok(ast::Literal::Num(v)),
        Err(_) => {
            ctx.errors.push(E::InvalidNumLiteral(span));
            return Err(())
        }
    }
}

fn convert_str(span: Span, ctx: &mut Ctx) -> Result<ast::Literal, ()> {
    let mut chars = ctx.buf.span(span).chars()
        .zip((span.start()..).map(|n| Span::new(n, n + 1)));

    let mut escaped = String::new();
    let mut is_valid = true;

    while let Some((ch, span)) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some((ch, span)) => {
                    escaped.push(match ch {
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        '0' => '\0',
                        '\'' => '\'',
                        '\"' => '\"',
                        '\\' => '\\',
                        _ => {
                            ctx.errors.push(E::UnknownStringEscape(span));
                            is_valid = false;
                            continue
                        }
                    });
                }
                None => {
                    ctx.errors.push(E::UnknownStringEscape(span));
                    return Err(());
                }
            }
            continue;
        }

        match ch {
            '\r' | '\n' => {
                ctx.errors.push(E::InvalidStringChar(span));
                is_valid = false;
            }
            _ => {
                escaped.push(ch);
            }
        }
    }

    if is_valid {
        Ok(ast::Literal::Str(escaped))
    } else {
        Err(())
    }
}
