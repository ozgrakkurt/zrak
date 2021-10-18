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

    pub fn get_next(&mut self) -> Result<Token, Error> {
        let c = match self.skip_whitespace() {
            Some(c) => c,
            None => return Ok(Token::Eof),
        };

        let token = match c {
            '?' => Token::Operator(Operator::QMark),
            '=' => self.assign(),
            c => todo!(),
        };

        Ok(token)
    }

    fn assign(&mut self) -> Token {
        unimplemented!();
    }

    fn advance_if(&mut self, expected: char) -> Option<()> {
        assert_ne!(expected, '\n');

        if self.input.peek()?.1 == expected {
            let (i, _) = self.input.next().unwrap();
            self.pos.col += 1;
            self.pos.idx = i;
            return Some(());
        }

        None
    }

    fn advance(&mut self) -> Option<char> {
        match self.input.next() {
            Some((i, c)) => {
                if c == '\n' {
                    self.pos.line += 1;
                    self.pos.col = 1;
                } else {
                    self.pos.col += 1;
                }
                self.pos.idx = i;
                Some(c)
            }
            None => None,
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
