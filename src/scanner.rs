use crate::error::Error;
use crate::str_interner::Interner;
use crate::token::{
    Assign, CmpOp, Delimiter, FactorOp, Keyword, Literal, Operator, ShiftOp, TermOp, Token,
};
use std::iter::Peekable;
use std::str::{CharIndices, FromStr};

pub struct Scanner<'a> {
    input: Peekable<CharIndices<'a>>,
    input_str: &'a str,
    pos: Pos,
    interner: &'a mut Interner,
    buf: Option<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str, interner: &'a mut Interner) -> Scanner<'a> {
        Scanner {
            input: input.char_indices().peekable(),
            input_str: input,
            pos: Pos::default(),
            interner,
            buf: None,
        }
    }

    pub fn putback(&mut self, token: Token) {
        assert!(self.buf.is_none());
        self.buf = Some(token);
    }

    pub fn peek_next(&mut self) -> Result<Token, Error> {
        let token = self.get_next()?;
        self.buf = Some(token);
        Ok(token)
    }

    pub fn get_next(&mut self) -> Result<Token, Error> {
        if let Some(token) = self.buf.take() {
            return Ok(token);
        }

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
            ')' => Token::Delimiter(Delimiter::ClosePrnth),
            '{' => Token::Delimiter(Delimiter::OpenCurly),
            '}' => Token::Delimiter(Delimiter::CloseCurly),
            '.' => Token::Delimiter(Delimiter::Dot),
            ',' => Token::Delimiter(Delimiter::Comma),
            '\'' => self.character()?,
            '"' => self.string()?,
            ':' => Token::Delimiter(Delimiter::Colon),
            ';' => Token::Delimiter(Delimiter::Semicolon),
            _ => {
                if c.is_ascii_digit() {
                    self.number()?
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
                None => break self.input_str.len(),
            };

            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance().unwrap();
            } else {
                break self.pos.idx + c.len_utf8();
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
            "map" => Token::Keyword(Keyword::Map),
            "new" => Token::Keyword(Keyword::New),
            _ => Token::Ident(self.interner.intern_str(ident)),
        }
    }

    fn number(&mut self) -> Result<Token, Error> {
        let mut had_dot = false;
        let start = self.pos.idx;
        let end = loop {
            let c = match self.input.peek() {
                Some(c) => c.1,
                None => break self.input_str.len(),
            };

            if c.is_ascii_digit() {
                self.advance().unwrap();
            } else if c == '.' {
                if had_dot {
                    break self.pos.idx + c.len_utf8();
                } else {
                    had_dot = true;
                    self.advance().unwrap();
                }
            } else {
                break self.pos.idx + c.len_utf8();
            }
        };

        let num = &self.input_str[start..end];

        let num = if had_dot {
            Token::Literal(Literal::Float(
                f64::from_str(num).map_err(Error::ParseFloatError)?,
            ))
        } else {
            Token::Literal(Literal::Int(
                i64::from_str(num).map_err(Error::ParseIntError)?,
            ))
        };

        Ok(num)
    }

    fn string(&mut self) -> Result<Token, Error> {
        let mut buf = String::new();

        loop {
            let c = match self.advance() {
                Some(c) => c,
                None => return Err(Error::UnclosedStringLiteral),
            };

            match c {
                '"' => {
                    let s = self.interner.intern(buf);
                    return Ok(Token::Literal(Literal::Str(s)));
                }
                '\\' => {
                    let c = match match self.advance() {
                        Some(c) => c,
                        None => return Err(Error::InvalidEscapeSequence),
                    } {
                        'n' => '\n',
                        't' => '\t',
                        '"' => '"',
                        '\\' => '\\',
                        _ => return Err(Error::InvalidEscapeSequence),
                    };
                    buf.push(c);
                }
                _ => buf.push(c),
            }
        }
    }

    fn character(&mut self) -> Result<Token, Error> {
        let c = match self.advance() {
            Some(c) => c,
            None => return Err(Error::UnclosedCharLiteral),
        };

        match c {
            '\\' => {
                let c = match match self.advance() {
                    Some(c) => c,
                    None => return Err(Error::InvalidEscapeSequence),
                } {
                    'n' => '\n',
                    't' => '\t',
                    '\'' => '\'',
                    '\\' => '\\',
                    _ => return Err(Error::InvalidEscapeSequence),
                };
                match self.advance() {
                    Some(q) if q == '\'' => Ok(Token::Literal(Literal::Char(c))),
                    _ => Err(Error::UnclosedCharLiteral),
                }
            }
            '\'' => Err(Error::EmptyCharLiteral),
            _ => match self.advance() {
                Some(q) if q == '\'' => Ok(Token::Literal(Literal::Char(c))),
                _ => Err(Error::UnclosedCharLiteral),
            },
        }
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
                    Token::Operator(Operator::Shift(ShiftOp::Right))
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
                    Token::Operator(Operator::Shift(ShiftOp::Left))
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
            Token::Operator(Operator::Factor(FactorOp::Mod))
        }
    }

    fn div(&mut self) -> Token {
        if self.advance_if('=').is_some() {
            Token::Assign(Assign::Div)
        } else {
            Token::Operator(Operator::Factor(FactorOp::Div))
        }
    }

    fn mul(&mut self) -> Token {
        if self.advance_if('=').is_some() {
            Token::Assign(Assign::Mul)
        } else {
            Token::Operator(Operator::Factor(FactorOp::Mul))
        }
    }

    fn sub(&mut self) -> Token {
        if self.advance_if('=').is_some() {
            Token::Assign(Assign::Sub)
        } else {
            Token::Operator(Operator::Term(TermOp::Sub))
        }
    }

    fn add(&mut self) -> Token {
        if self.advance_if('=').is_some() {
            Token::Assign(Assign::Add)
        } else {
            Token::Operator(Operator::Term(TermOp::Add))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool() {
        let mut interner = Interner::new();
        let mut scanner = Scanner::new("true false", &mut interner);
        assert_eq!(
            scanner.get_next().unwrap(),
            Token::Literal(Literal::Bool(true))
        );
        assert_eq!(
            scanner.get_next().unwrap(),
            Token::Literal(Literal::Bool(false))
        );
    }

    #[test]
    fn test_string() {
        let mut interner = Interner::new();
        let mut scanner = Scanner::new(
            r#" "\n\t\"hello
123" "#,
            &mut interner,
        );
        let s = match scanner.get_next().unwrap() {
            Token::Literal(Literal::Str(s)) => interner.lookup(s).unwrap(),
            _ => panic!("failed to parse string literal"),
        };
        assert_eq!(s, "\n\t\"hello\n123");
    }

    #[test]
    fn test_number() {
        let mut interner = Interner::new();
        let mut scanner = Scanner::new("5.5. 38", &mut interner);
        assert_eq!(
            scanner.get_next().unwrap(),
            Token::Literal(Literal::Float(5.5))
        );
        assert_eq!(
            scanner.get_next().unwrap(),
            Token::Delimiter(Delimiter::Dot)
        );
        assert_eq!(
            scanner.get_next().unwrap(),
            Token::Literal(Literal::Int(38))
        );
    }
}
