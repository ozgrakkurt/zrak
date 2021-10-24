use crate::ast;
use crate::error::{Error, Result};
use crate::scanner::Scanner;
use crate::str_interner::Interner;

pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse(input: &str) -> Result<ast::Program> {
        let mut interner = Interner::new();
        let scanner = Scanner::new(input, &mut interner);
        let mut ps = ParseState { scanner };
        ps.program()
    }
}

pub struct ParseState<'a> {
    scanner: Scanner<'a>,
}

impl<'a> ParseState<'a> {
    pub fn program(&mut self) -> Result<ast::Program> {
        todo!()
    }
}

impl Default for Parser {
    fn default() -> Parser {
        Parser::new()
    }
}
