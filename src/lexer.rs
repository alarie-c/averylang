use crate::token::{TextSpan, Token, TokenKind};

pub struct Lexer<'a> {
    src: &'a [u8],
    idx: usize,
    output: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src: src.as_bytes(), idx: 0usize, output: Vec::new() }
    }

    pub fn scan(&mut self) -> &Vec<Token> {       
        loop {
            // Break the loop if we reach the end
            if self.idx >= self.src.len() { break; }

            // Match current chars
            match &self.src[self.idx..] {
                // Ignore whitespace & newline chars
                [b' ', ..] | [b'\t', ..] | [b'\n', ..] | [b'\r', ..] => self.idx += 1,
                
                // Grouping operators
                [b'(', ..] => self.token(TokenKind::LPar, self.idx, "("),
                [b')', ..] => self.token(TokenKind::RPar, self.idx, ")"),
                [b'[', ..] => self.token(TokenKind::LBrac, self.idx, "["),
                [b']', ..] => self.token(TokenKind::RBrac, self.idx, "]"),
                [b'{', ..] => self.token(TokenKind::LCurl, self.idx, "{"),
                [b'}', ..] => self.token(TokenKind::RCurl, self.idx, "}"),
                
                // Other
                _ => panic!("! Bad token !"),
            }
        }

        &self.output
    }

    fn token(&mut self, kind: TokenKind, begin: usize, str: &str) {
        self.output.push(Token { kind, span: TextSpan::new(begin, str) });
        self.idx += str.len();
    }
}