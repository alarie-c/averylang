use crate::token::{TextSpan, Token, TokenKind};

pub struct Lexer<Iter: Iterator<Item = char>> {
    input: Iter,
    output: Vec<Token>,
    chr: char,
    idx: usize,
}

impl<Iter: Iterator<Item = char>> Lexer<Iter> {
    fn new(input: Iter) -> Self {
        Self {
            input,
            output: Vec::new(),
            chr: char::default(),
            idx: 0usize,
        }
    }
}