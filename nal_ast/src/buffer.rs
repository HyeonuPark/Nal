use std::str::FromStr;

use ast::common::Span;
use ast::module::Module;
use parser::{parse, ParseError};

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
    type Err = ParseError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let module = parse(src)?;
        let line_pos = parse_line_pos(src);

        Ok(Self {
            src: src.into(),
            line_pos,
            module,
        })
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
mod test_offset_pos {
    use super::*;

    fn dummy(src: &str) -> SourceBuffer {
        SourceBuffer {
            src: src.into(),
            module: unsafe { ::std::mem::zeroed() },
            line_pos: parse_line_pos(src),
        }
    }

    #[test]
    fn test_srcbuf_offset() {
        let src = "
foo
bar
baz
        ".trim();
        assert_eq!(src.len(), 11);

        let srcbuf = dummy(src);
        assert_eq!(srcbuf.line_pos, vec![3, 7, 10]);

        assert_eq!(srcbuf.offset_byte_pos(0), (0, 0));
        assert_eq!(srcbuf.offset_byte_pos(1), (0, 1));
        assert_eq!(srcbuf.offset_byte_pos(3), (0, 3));
        assert_eq!(srcbuf.offset_byte_pos(4), (1, 0));
        assert_eq!(srcbuf.offset_byte_pos(5), (1, 1));
        assert_eq!(srcbuf.offset_byte_pos(10), (2, 2));
    }
}
