// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use super::lookahead::{AheadLooker, Lookahead};

struct SourceReader {
    src: String,
    pos: usize,
}

impl SourceReader {
    fn new(src: String) -> SourceReader {
        SourceReader { src, pos: 0 }
    }
}

impl Iterator for SourceReader {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        (self.pos < self.src.bytes().len()).then(|| {
            let char = self.src[self.pos..].chars().next().unwrap();
            self.pos += char.len_utf8();
            char
        })
    }
}

pub struct Lexer {
    sr: AheadLooker<SourceReader>,
}

impl Lexer {
    pub fn new(src: String) -> Lexer {
        Lexer { sr: SourceReader::new(src).ahead_looker() }
    }
}
