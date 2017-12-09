use codebuf::{Node, Span};
use nal_ast::ast;
use parse_tree as pt;
use super::{Convert, Ctx, Error as E};

impl Convert<Node<ast::Literal>> for Node<pt::Literal> {
    fn convert(&self, ctx: &mut Ctx) -> Result<Node<ast::Literal>, ()> {
        Ok(Node::new(
            self.span,
            match **self {
                pt::Literal::Bool => {
                    let parsed = ctx.buf.span(self).parse::<bool>();

                    match parsed {
                        Ok(v) => ast::Literal::Bool(v),
                        Err(_) => {
                            ctx.errors.push(E::InvalidBoolLiteral(self.span));
                            return Err(())
                        }
                    }
                }
                pt::Literal::Num => {
                    let escaped: String = ctx.buf.span(self).chars()
                        .filter(|&c| !c.is_whitespace() && c != '_')
                        .collect();
                    let parsed = escaped.parse::<f64>();

                    match parsed {
                        Ok(v) => ast::Literal::Num(v),
                        Err(_) => {
                            ctx.errors.push(E::InvalidNumLiteral(self.span));
                            return Err(())
                        }
                    }
                }
                pt::Literal::Str => {
                    let mut chars = ctx.buf.span(self).chars()
                        .zip(
                            (self.span.start()..)
                                .map(|n| Span::new(n, n + 1))
                        );
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
                                            ctx.errors.push(
                                                E::UnknownStringEscape(span)
                                            );
                                            is_valid = false;
                                            continue
                                        }
                                    });
                                }
                                None => {
                                    ctx.errors.push(
                                        E::UnknownStringEscape(span)
                                    );
                                    return Err(());
                                }
                            }
                            continue;
                        }

                        match ch {
                            '\n' => {
                                ctx.errors.push(
                                    E::InvalidStringChar(span)
                                );
                                is_valid = false;
                            }
                            _ => {
                                escaped.push(ch);
                            }
                        }
                    }

                    if is_valid {
                        ast::Literal::Str(escaped)
                    } else {
                        return Err(())
                    }
                }
            }
        ))
    }
}
