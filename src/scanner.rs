use crate::error::Error;
use crate::str_interner::Interner;
use crate::token::{Assign, CmpOp, Delimiter, Keyword, Literal, Operator, Token};
use std::iter::Peekable;
use std::str::CharIndices;

pub struct Scanner<'a> {
    input: Peekable<CharIndices<'a>>,
    input_str: &'a str,
    pos: Pos,
    interner: &'a mut Interner,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str, interner: &'a mut Interner) -> Scanner<'a> {
        Scanner {
            input: input.char_indices().peekable(),
            input_str: input,
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
            '+' => self.add(),
            '-' => self.sub(),
            '*' => self.mul(),
            '/' => self.div(),
            '%' => self.rem(),
            '|' => self.bit_or(),
            '&' => self.bit_and(),
            '^' => self.bit_xor(),
            '<' => self.less(),
            '>' => self.greater(),
            '!' => self.not(),
            '[' => Token::Delimiter(Delimiter::OpenBrkt),
            ']' => Token::Delimiter(Delimiter::CloseBrkt),
            '(' => Token::Delimiter(Delimiter::OpenPrnth),
            ')' => Token::Delimiter(Delimiter::CloseBrkt),
            '{' => Token::Delimiter(Delimiter::OpenCurly),
            '}' => Token::Delimiter(Delimiter::CloseCurly),
            '.' => Token::Delimiter(Delimiter::Dot),
            ',' => Token::Delimiter(Delimiter::Comma),
            '\'' => self.character()?,
            '"' => self.string()?,
            _ => {
                if c.is_ascii_digit() {
                    self.number()
                } else if c.is_ascii_alphabetic() || c == '_' {
                    self.ident()
                } else {
                    return Err(Error::UnexpectedCharacter(c));
                }
            }
        };

        Ok(token)
    }

    fn ident(&mut self) -> Token {
        let start = self.pos.idx;
        let end = loop {
            let c = match self.input.peek() {
                Some(c) => c.1,
                None => break self.pos.idx,
            };

            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance().unwrap();
            } else {
                break self.pos.idx;
            }
        };

        let ident = &self.input_str[start..end];

        match ident {
            "true" => Token::Literal(Literal::Bool(true)),
            "false" => Token::Literal(Literal::Bool(false)),
            "null" => Token::Literal(Literal::Null),
            "in" => Token::Keyword(Keyword::In),
            "for" => Token::Keyword(Keyword::For),
            "while" => Token::Keyword(Keyword::While),
            "loop" => Token::Keyword(Keyword::Loop),
            "if" => Token::Keyword(Keyword::If),
            "else" => Token::Keyword(Keyword::Else),
            "struct" => Token::Keyword(Keyword::Struct),
            "fn" => Token::Keyword(Keyword::Fn),
            "let" => Token::Keyword(Keyword::Let),
            "self" => Token::Keyword(Keyword::SelfKw),
            "return" => Token::Keyword(Keyword::Return),
            "break" => Token::Keyword(Keyword::Break),
            _ => Token::Ident(self.interner.intern_str(ident)),
        }
    }

    fn number(&mut self) -> Token {
        todo!()
    }

    fn string(&mut self) -> Result<Token, Error> {
        todo!()
    }

    fn character(&mut self) -> Result<Token, Error> {
        todo!()
    }

    fn not(&mut self) -> Token {
        if self.advance_if('=').is_some() {
            Token::Operator(Operator::Cmp(CmpOp::NotEq))
        } else {
            Token::Operator(Operator::Not)
        }
    }

    fn greater(&mut self) -> Token {
        match match self.input.peek() {
            Some(c) => c.1,
            None => return Token::Operator(Operator::Cmp(CmpOp::Greater)),
        } {
            '=' => {
                self.advance().unwrap();
                Token::Operator(Operator::Cmp(CmpOp::GreaterEq))
            }
            '>' => {
                self.advance().unwrap();
                if self.advance_if('=').is_some() {
                    Token::Assign(Assign::RightShift)
                } else {
                    Token::Operator(Operator::RightShift)
                }
            }
            _ => Token::Operator(Operator::Cmp(CmpOp::Greater)),
        }
    }

    fn less(&mut self) -> Token {
        match match self.input.peek() {
            Some(c) => c.1,
            None => return Token::Operator(Operator::Cmp(CmpOp::Less)),
        } {
            '=' => {
                self.advance().unwrap();
                Token::Operator(Operator::Cmp(CmpOp::LessEq))
            }
            '<' => {
                self.advance().unwrap();
                if self.advance_if('=').is_some() {
                    Token::Assign(Assign::LeftShift)
                } else {
                    Token::Operator(Operator::LeftShift)
                }
            }
            _ => Token::Operator(Operator::Cmp(CmpOp::Less)),
        }
    }

    fn bit_xor(&mut self) -> Token {
        if self.advance_if('=').is_some() {
            Token::Assign(Assign::BitXor)
        } else {
            Token::Operator(Operator::BitXor)
        }
    }

    fn bit_and(&mut self) -> Token {
        match match self.input.peek() {
            Some(c) => c.1,
            None => return Token::Operator(Operator::BitAnd),
        } {
            '&' => {
                self.advance().unwrap();
                Token::Operator(Operator::LogicAnd)
            }
            '=' => {
                self.advance().unwrap();
                Token::Assign(Assign::BitAnd)
            }
            _ => Token::Operator(Operator::BitAnd),
        }
    }

    fn bit_or(&mut self) -> Token {
        match match self.input.peek() {
            Some(c) => c.1,
            None => return Token::Operator(Operator::BitOr),
        } {
            '|' => {
                self.advance().unwrap();
                Token::Operator(Operator::LogicOr)
            }
            '=' => {
                self.advance().unwrap();
                Token::Assign(Assign::BitOr)
            }
            _ => Token::Operator(Operator::BitOr),
        }
    }

    fn rem(&mut self) -> Token {
        if self.advance_if('=').is_some() {
            Token::Assign(Assign::Mod)
        } else {
            Token::Operator(Operator::Mod)
        }
    }

    fn div(&mut self) -> Token {
        if self.advance_if('=').is_some() {
            Token::Assign(Assign::Div)
        } else {
            Token::Operator(Operator::Div)
        }
    }

    fn mul(&mut self) -> Token {
        if self.advance_if('=').is_some() {
            Token::Assign(Assign::Mul)
        } else {
            Token::Operator(Operator::Mul)
        }
    }

    fn sub(&mut self) -> Token {
        if self.advance_if('=').is_some() {
            Token::Assign(Assign::Sub)
        } else {
            Token::Operator(Operator::Sub)
        }
    }

    fn add(&mut self) -> Token {
        if self.advance_if('=').is_some() {
            Token::Assign(Assign::Add)
        } else {
            Token::Operator(Operator::Add)
        }
    }

    fn assign(&mut self) -> Token {
        if self.advance_if('=').is_some() {
            Token::Operator(Operator::Cmp(CmpOp::Eq))
        } else {
            Token::Assign(Assign::Assign)
        }
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
