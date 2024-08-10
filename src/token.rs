use std::ops::{Bound, Range, RangeBounds};

// Stores beginning and ending information for a token
#[derive(Debug)]
pub struct TextSpan {
    pub begin: usize,
    pub end: usize,
}

// Methods for creating spans and getting the length
impl TextSpan {
    pub fn new(begin: usize, len: usize) -> Self {
        TextSpan { begin, end:  begin + usize::saturating_sub(len, 1) }
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
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

// Enumeration of every token type and the data stored with it
#[derive(Debug)]
pub enum TokenKind {
    // Grouping
    RPar,
    LPar,
    RBrac,
    LBrac,
    RCurl,
    LCurl,

    // Operators
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Modulo,
    Ampersand,
    Exponent,
    Star,
    Slash,
    SlashSlash,
    Hash,
    Arrow,
    Logger,

    // Comparisons
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    More,
    MoreEqual,
    Bang,
    BangEqual,

    // Literals
    Literal(String),
    Identifier(String),

    // Keywords
    If,
    Else,
    Elif,
    While,
    For,

    // Other
    EndOfFile,
}