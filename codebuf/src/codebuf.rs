use span::Spanned;

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

    /// Corresponding source code of given span
    pub fn span<S: Spanned + ?Sized>(&self, spanned: &S) -> &str {
        let span = spanned.span();

        &self.code[span.start()..span.end()]
    }

    /// Line informatin of this byte offset
    ///
    /// Returns (line number, line offset)
    /// both starts from 0
    pub fn line_of(&self, offset: usize) -> (usize, usize) {
        assert!(offset < self.code.len());

        let line_num = match self.line_table.binary_search(&offset) {
            Ok(n) | Err(n) => n,
        };
        let line_offset = match line_num {
            0 => 0,
            _ => self.line_table[line_num - 1] + 1,
        };

        (line_num, line_offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_of() {
        let code = "\
            ab\n\
            cde\n\
            fghij\
        ";
        let buf = CodeBuf::new(code);

        assert_eq!(buf.line_table, vec![2, 6, 12].into());

        assert_eq!(buf.line_of(1), (0, 0));
        assert_eq!(buf.line_of(6), (1, 3));
    }

    #[test]
    #[should_panic]
    fn test_fail_line_of() {
        let code = "abc";
        let buf = CodeBuf::new(code);
        buf.line_of(3);
    }
}
