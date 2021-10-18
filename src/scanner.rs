use crate::error::Error;
use crate::str_interner::Interner;
use crate::token::{Operator, Token};
use std::iter::Peekable;
use std::str::CharIndices;

pub struct Scanner<'a> {
    input: Peekable<CharIndices<'a>>,
    pos: Pos,
    interner: &'a mut Interner,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str, interner: &'a mut Interner) -> Scanner<'a> {
        Scanner {
            input: input.char_indices().peekable(),
            pos: Pos::default(),
            interner,
        }
    }

    fn skip_whitespace(&mut self) -> Option<char> {
        for (i, c) in &mut self.input {
            if c == '\n' {
                self.pos.line += 1;
                self.pos.col = 1;
            } else {
                self.pos.col += 1;
                if !c.is_whitespace() {
                    self.pos.idx = i;
                    return Some(c);
                }
            }
        }

        None
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = match self.skip_whitespace()? {
            '?' => Token::Operator(Operator::QMark),
            c => todo!(),
        };

        Some(Ok(token))
    }
}

pub struct Pos {
    line: u32,
    col: u32,
    idx: usize,
}

impl Default for Pos {
    fn default() -> Pos {
        Pos {
            line: 1,
            col: 1,
            idx: 0,
        }
    }
}
