use std::str::FromStr;
use std::ops::Deref;

use ast::common::Span;
use ast::module::Module;
use parse::parse;
use check::check;
use Report;

#[derive(Debug)]
pub struct SourceBuffer {
    src: String,
    line_pos: Vec<usize>,
    module: Module,
}

fn parse_line_pos(src: &str) -> Vec<usize> {
    let mut line_pos = Vec::new();

    for (idx, ch) in src.bytes().enumerate() {
        if ch == b'\n' {
            line_pos.push(idx);
        }
    }

    if let Some(&ch) = src.as_bytes().last() {
        if ch != b'\n' {
            line_pos.push(src.len() - 1);
        }
    }

    line_pos
}

impl FromStr for SourceBuffer {
    type Err = Report;

    fn from_str(src: &str) -> Result<Self, Report> {
        let module = parse(src)?;
        check(&module)?;

        Ok(Self {
            src: src.to_string(),
            line_pos: parse_line_pos(src),
            module,
        })
    }
}

impl Deref for SourceBuffer {
    type Target = Module;

    fn deref(&self) -> &Self::Target {
        &self.module
    }
}

impl SourceBuffer {
    pub fn span_content(&self, span: Span) -> &str {
        let Span(start, end) = span;
        &self.src[start..end]
    }

    pub fn offset_line(&self, offset: usize) -> usize {
        assert!(offset < self.src.len());
        match self.line_pos.binary_search(&offset) {
            Ok(n) => n,
            Err(n) => n,
        }
    }

    pub fn offset_byte_pos(&self, offset: usize) -> (usize, usize) {
        let line = self.offset_line(offset);
        let line_offset = match line {
            0 => 0,
            n => self.line_pos[n - 1] + 1,
        };
        let column = offset - line_offset;
        (line, column)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_srcbuf_offset() {
        let src = "
            true\n\
            true\n\
            false
        ".trim();
        assert_eq!(src.len(), 15);

        let srcbuf: SourceBuffer = src.parse().unwrap();
        assert_eq!(srcbuf.line_pos, vec![4, 9, 14]);

        assert_eq!(srcbuf.offset_byte_pos(0), (0, 0));
        assert_eq!(srcbuf.offset_byte_pos(1), (0, 1));
        assert_eq!(srcbuf.offset_byte_pos(4), (0, 4));
        assert_eq!(srcbuf.offset_byte_pos(5), (1, 0));
        assert_eq!(srcbuf.offset_byte_pos(6), (1, 1));
        assert_eq!(srcbuf.offset_byte_pos(13), (2, 3));
    }

    #[test]
    fn test_span_content() {
        let src = "
            333
            true && -false
            5+ 6
        ".trim();
        let srcbuf: SourceBuffer = src.parse().unwrap();

        assert_eq!(srcbuf.span_content(srcbuf.body[0].span), "333");
        assert_eq!(srcbuf.span_content(srcbuf.body[1].span), "true && -false");
        assert_eq!(srcbuf.span_content(srcbuf.body[2].span), "5+ 6");
    }
}
