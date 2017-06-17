use std::rc::Rc;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SrcFile {
    content: String,
    acc_line_len: Vec<usize>,
}

impl SrcFile {
    pub fn new(content: String) -> Self {
        let mut acc_line_len = vec![0];

        for (idx, byte) in content.bytes().enumerate() {
            if byte == '\n' as u8 {
                acc_line_len.push(idx + 1);
            }
        }

        SrcFile {
            content,
            acc_line_len,
        }
    }

    pub fn position(&self, offset: usize) -> (usize, usize) {
        assert!(offset < self.content.len());

        let line_cnt = match self.acc_line_len.binary_search(&(offset + 1)) {
            Ok(n) => n,
            Err(n) => n,
        };

        let line_offset = self.acc_line_len[line_cnt];
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
        self.content.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct SrcPos {
    pub file: Rc<SrcFile>,
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
