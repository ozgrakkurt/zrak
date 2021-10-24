use crate::str_interner::IntStr;
use crate::token::Token;
use std::num;
use std::result::Result as StdResult;

#[derive(Debug)]
pub enum Error {
    UnexpectedCharacter(char),
    ParseFloatError(num::ParseFloatError),
    ParseIntError(num::ParseIntError),
    UnclosedStringLiteral,
    UnclosedCharLiteral,
    EmptyCharLiteral,
    InvalidEscapeSequence,
    UnexpectedToken(Token),
    MethodDefinedTwice(IntStr),
    UnassignableExpression,
}

pub type Result<T> = StdResult<T, Error>;
