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
}

pub type Result<T> = StdResult<T, Error>;
