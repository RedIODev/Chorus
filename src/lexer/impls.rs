use std::rc::Rc;

use super::{Keyword, KeywordToken, SourcePosition, Token};

impl Clone for SourcePosition {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for SourcePosition {}

impl SourcePosition {
    pub fn new(row: u32, column: u32) -> Self {
        Self { row, column }
    }
}

impl From<(u32, u32)> for SourcePosition {
    fn from(value: (u32, u32)) -> Self {
       SourcePosition::new(value.0, value.1)
    }
}

impl Token for KeywordToken {
    fn raw(&self) -> Rc<str> {
        todo!()
    }

    fn source_position(&self) -> SourcePosition {
        self.source_position
    }
}

impl KeywordToken {
    pub fn new(keyword: Keyword, source_position: SourcePosition) -> Self {
        Self {
            keyword,
            source_position,
        }
    }
}
