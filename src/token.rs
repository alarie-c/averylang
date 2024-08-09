use std::ops::{Bound, Range, RangeBounds};

// Stores beginning and ending information for a token
pub struct TextSpan {
    begin: usize,
    end: usize,
}

// Methods for creating spans and getting the length
impl TextSpan {
    pub fn new(begin: usize, end: usize) -> Self {
        TextSpan { begin, end }
    }

    pub fn len(&self) -> usize {
        self.end - self.begin
    }
}

// Implement RangeBounds for compatibility with stdlib
impl RangeBounds<usize> for TextSpan {
    fn start_bound(&self) -> std::ops::Bound<&usize> {
        Bound::Included(&self.begin)
    }

    fn end_bound(&self) -> std::ops::Bound<&usize> {
        Bound::Excluded(&self.end)
    }
}

// Implement methods to convert span to range
impl Into<Range<usize>> for TextSpan {
    fn into(self) -> Range<usize> {
        self.begin..self.end
    }
}

// Store the kind and span of a single token
pub struct Token {
    kind: TokenKind,
    span: TextSpan,
}

// Enumeration of every token type and the data stored with it
pub enum TokenKind {
    RPar,
    LPar,
}