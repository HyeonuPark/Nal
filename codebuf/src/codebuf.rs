use span::Node;

/// Codebuf contains source code and its line information
#[derive(Debug)]
pub struct CodeBuf {
    code: Box<str>,
    line_table: Box<[usize]>,
}

impl CodeBuf {
    pub fn new<C: Into<Box<str>>>(code: C) -> Self {
        let code = code.into();

        let line_table = code.bytes()
            .enumerate()
            .filter(|&(_, by)| by == b'\n')
            .map(|(idx, _)| idx)
            .chain(Some(code.len()))
            .collect::<Vec<_>>()
            .into_boxed_slice();

        CodeBuf {
            code,
            line_table,
        }
    }

    /// Full code this CodeBuf contains
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Corresponding source code of given node
    pub fn span<T>(&self, node: &Node<T>) -> &str {
        let span = node.span;

        &self.code[span.start()..span.end()]
    }

    /// Line information of this byte offset
    ///
    /// Both line count and offset are starts from 0
    pub fn line_of(&self, offset: usize) -> Line {
        assert!(offset < self.code.len());

        let line_count = match self.line_table.binary_search(&offset) {
            Ok(n) | Err(n) => n,
        };
        let line_offset = match line_count {
            0 => 0,
            _ => self.line_table[line_count - 1] + 1,
        };

        Line {
            count: line_count,
            offset: line_offset,
        }
    }
}

/// Line information
///
/// Both line count and offset are starts from 0
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Line {
    count: usize,
    offset: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(count: usize, offset: usize) -> Line {
        Line { count, offset }
    }

    #[test]
    fn test_line_of() {
        let code = "\
            ab\n\
            cde\n\
            fghij\
        ";
        let buf = CodeBuf::new(code);

        assert_eq!(buf.line_table, vec![2, 6, 12].into());

        assert_eq!(buf.line_of(1), line(0, 0));
        assert_eq!(buf.line_of(6), line(1, 3));
    }

    #[test]
    fn test_crlf_line_of() {
        let code = "\
            a\r\n\
            cd\r\n\
            fghij\
        ";
        let buf = CodeBuf::new(code);

        assert_eq!(buf.line_table, vec![2, 6, 12].into());

        assert_eq!(buf.line_of(1), line(0, 0));
        assert_eq!(buf.line_of(6), line(1, 3));
    }

    #[test]
    #[should_panic]
    fn test_fail_line_of() {
        let code = "abc";
        let buf = CodeBuf::new(code);
        buf.line_of(3);
    }
}
