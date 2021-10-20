use std::num;

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
