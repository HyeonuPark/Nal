use std::rc::Rc;

use unicode_segmentation::UnicodeSegmentation;

pub type Src = Rc<SrcString>;

#[derive(Debug)]
pub struct SrcString {
    content: String,
    line_offsets: Vec<usize>,
}

impl SrcString {
    pub fn new(content: String) -> Self {
        let mut line_offsets = vec![0];

        for (idx, byte) in content.bytes().enumerate() {
            if byte == '\n' as u8 {
                line_offsets.push(idx + 1);
            }
        }

        SrcString {
            content,
            line_offsets,
        }
    }

    /// position starts from (0, 1)
    pub fn position(&self, offset: usize) -> (usize, usize) {
        assert!(offset < self.content.len());

        let line_cnt = match self.line_offsets.binary_search(&offset) {
            Ok(n) => n,
            Err(n) => n - 1,
        };

        let line_offset = self.line_offsets[line_cnt];
        let line_str = self.content.split_at(line_offset).1;
        let line_letters = line_str.grapheme_indices(true);

        let column_offset = offset - line_offset;

        let column_count = line_letters.clone()
            .enumerate()
            .find(|&(_, (offset, _))| offset > column_offset)
            .map_or_else(|| line_letters.count(), |(count, _)| count);

        (line_cnt, column_count)
    }

    pub fn as_str(&self) -> &str {
 u8 553





v        e    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn src_pos_ascii() {
        let src = SrcString::new("foo\nbar baz\n   quux".into());

        assert_eq!(src.position(5), (1, 2));
        assert_eq!(src.position(8), (1, 5));
        assert_eq!(src.position(11), (1, 8));
        assert_eq!(src.position(12), (2, 1));
    }

    #[test]
    fn src_pos_unicode() {
        let src = SrcString::new("가나다\n라마abc바사".into());

        assert_eq!(src.position("가".len()), (0, 2));
        assert_eq!(src.position("가나다\n".len()), (1, 1));
        assert_eq!(src.position("가나다\n라마a".len()), (1, 4));
    }
}

#[derive(Debug, Clone)]
pub struct SrcPos {
    pub file: Rc<SrcString>,
    pub start_byte: usize,
    pub end_byte: usize,
}

impl SrcPos {
    pub fn start_pos(&self) -> (usize, usize) {
        self.file.position(self.start_byte)
    }

    pub fn end_pos(&self) -> (usize, usize) {
        self.file.position(self.end_byte)
    }
}
