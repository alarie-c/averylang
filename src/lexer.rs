use std::str::from_utf8;

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
            if self.idx >= self.src.len() {
                self.token(TokenKind::EndOfFile, self.idx, ".");
                break;
            }

            // Match current chars
            match &self.src[self.idx..] {
                // Ignore whitespace & newline chars
                [b' ', ..] | [b'\t', ..] | [b'\n', ..] | [b'\r', ..] => self.idx += 1,
                
                // Grouping tokens
                [b'(', ..] => self.token(TokenKind::LPar, self.idx, "("),
                [b')', ..] => self.token(TokenKind::RPar, self.idx, ")"),
                [b'[', ..] => self.token(TokenKind::LBrac, self.idx, "["),
                [b']', ..] => self.token(TokenKind::RBrac, self.idx, "]"),
                [b'{', ..] => self.token(TokenKind::LCurl, self.idx, "{"),
                [b'}', ..] => self.token(TokenKind::RCurl, self.idx, "}"),

                // Operator tokens
                [b'/', b'/', ..] => self.token(TokenKind::SlashSlash, self.idx, "//"),
                [b'+', b'=', ..] => self.token(TokenKind::PlusEqual, self.idx, "+="),
                [b'-', b'=', ..] => self.token(TokenKind::MinusEqual, self.idx, "-="),
                [b'-', b'>', ..] => self.token(TokenKind::Arrow, self.idx, "->"),
                [b'+', ..] => self.token(TokenKind::Plus, self.idx, "+"),
                [b'-', ..] => self.token(TokenKind::Minus, self.idx, "-"),
                [b'%', ..] => self.token(TokenKind::Modulo, self.idx, "%"),
                [b'&', ..] => self.token(TokenKind::Ampersand, self.idx, "&"),
                [b'*', ..] => self.token(TokenKind::Star, self.idx, "*"),
                [b'/', ..] => self.token(TokenKind::Slash, self.idx, "/"),
                [b'#', ..] => self.token(TokenKind::Hash, self.idx, "#"),
                [b'$', ..] => self.token(TokenKind::Logger, self.idx, "$"),
                [b'^', ..] => self.token(TokenKind::Exponent, self.idx, "^"),
                
                // Comparison tokens
                [b'=', b'=', ..] => self.token(TokenKind::EqualEqual, self.idx, "=="),
                [b'<', b'=', ..] => self.token(TokenKind::LessEqual, self.idx, "<="),
                [b'>', b'=', ..] => self.token(TokenKind::MoreEqual, self.idx, ">="),
                [b'!', b'=', ..] => self.token(TokenKind::BangEqual, self.idx, "!="),
                [b'=', ..] => self.token(TokenKind::Equal, self.idx, "="),
                [b'<', ..] => self.token(TokenKind::Less, self.idx, "<"),
                [b'>', ..] => self.token(TokenKind::More, self.idx, ">"),
                [b'!', ..] => self.token(TokenKind::Bang, self.idx, "!"),

                // Literals
                [b'"', ..] => self.str_literal(),
                // [x, ..] if x.is_ascii_alphabetic() => {
                //     let begin = self.idx;
                //     let ident = &self.identifier();
                //     dbg!(ident);
                //     match ident.as_str() {
                //         "if" => self.token(TokenKind::If, begin, "if"),
                //         "else" => self.token(TokenKind::Else, begin, "else"),
                //         "elif" => self.token(TokenKind::Elif, begin, "elif"),
                //         "for" => self.token(TokenKind::For, begin, "for"),
                //         "while" => self.token(TokenKind::While, begin, "while"),
                //         _ => self.token(TokenKind::Identifier(ident.to_owned()), begin, ident),
                //     }
                // }

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

    fn str_literal(&mut self) {
        let begin = self.idx;
        let mut buf = String::from(self.src[self.idx] as char);
        self.idx += 1;
        dbg!(&buf);
        loop {
            if self.idx >= self.src.len() {
                panic!("! Non-terminating string literal !");
            } else if self.src[self.idx] == b'"' {
                buf.push(self.src[self.idx] as char);
                break;
            } else {
                buf.push(self.src[self.idx] as char);
                dbg!(&buf);
                self.idx += 1;
            }
        }

        // Push new token
        self.token(TokenKind::Literal(buf.to_owned()), begin, &buf);
    }

    // fn identifier(&mut self) -> String {
    //     let begin = self.idx;
    //     while self.idx + 1 < self.src.len() {
    //         if self.src[self.idx + 1].is_ascii_alphanumeric() || self.src[self.idx + 1] == b'_' {
    //             self.idx += 1;
    //         } else {
    //             break;
    //         }
    //     }

    //     from_utf8(&self.src[begin..self.idx + 1]).unwrap().to_owned()
    // }
}