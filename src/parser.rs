use crate::ast;
use crate::error::{Error, Result};
use crate::scanner::Scanner;
use crate::str_interner::{IntStr, Interner};
use crate::token::{Assign, Delimiter, Keyword, Token};
use std::collections::HashMap;

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

struct ParseState<'a> {
    scanner: Scanner<'a>,
}

impl<'a> ParseState<'a> {
    fn program(&mut self) -> Result<ast::Program> {
        let mut decls = Vec::new();
        while self.scanner.peek_next()? != Token::Eof {
            decls.push(self.decl()?);
        }
        Ok(ast::Program { decls })
    }

    fn decl(&mut self) -> Result<ast::Decl> {
        match self.scanner.get_next()? {
            Token::Keyword(Keyword::Struct) => self.struct_decl().map(ast::Decl::Struct),
            Token::Keyword(Keyword::Fn) => self.fun_decl().map(ast::Decl::Fun),
            Token::Keyword(Keyword::Let) => self.var_decl().map(ast::Decl::Var),
            token => {
                self.scanner.putback(token);
                self.stmt().map(ast::Decl::Stmt)
            }
        }
    }

    fn struct_decl(&mut self) -> Result<ast::StructDecl> {
        let ident = self.ident()?;

        self.consume(Token::Delimiter(Delimiter::OpenCurly))?;

        let mut methods = HashMap::new();

        loop {
            match self.scanner.get_next()? {
                Token::Keyword(Keyword::Fn) => {
                    let fun = self.fun_decl()?;
                    if methods.insert(fun.ident, fun).is_some() {
                        return Err(Error::MethodDefinedTwice(fun.ident));
                    }
                }
                Token::Delimiter(Delimiter::CloseCurly) => {
                    break;
                }
                token => return Err(Error::UnexpectedToken(token)),
            }
        }

        Ok(ast::StructDecl { ident, methods })
    }

    fn fun_decl(&mut self) -> Result<ast::FunDecl> {
        let ident = self.ident()?;

        self.consume(Token::Delimiter(Delimiter::OpenPrnth))?;
        let params = self.params()?;
        self.consume(Token::Delimiter(Delimiter::OpenPrnth))?;

        let block = self.block()?;

        Ok(ast::FunDecl {
            ident,
            params,
            block,
        })
    }

    fn params(&mut self) -> Result<Vec<IntStr>> {
        let mut params = Vec::new();

        loop {
            let ident = match self.scanner.get_next()? {
                Token::Ident(ident) => ident,
                token => {
                    self.scanner.putback(token);
                    break;
                }
            };

            match self.scanner.get_next()? {
                Token::Delimiter(Delimiter::Comma) => params.push(ident),
                token => {
                    self.scanner.putback(token);
                    params.push(ident);
                    break;
                }
            }
        }

        Ok(params)
    }

    fn var_decl(&mut self) -> Result<ast::VarDecl> {
        let ident = self.ident()?;

        self.consume(Token::Assign(Assign::Assign))?;

        let expr = self.expr()?;

        self.consume(Token::Delimiter(Delimiter::Semicolon))?;

        Ok(ast::VarDecl { ident, expr })
    }

    fn stmt(&mut self) -> Result<ast::Stmt> {
        match self.scanner.peek_next()? {
            Token::Keyword(Keyword::Return) => {
                self.scanner.get_next()?;
                if self.scanner.peek_next()? == Token::Delimiter(Delimiter::Semicolon) {
                    self.scanner.get_next()?;
                    Ok(ast::Stmt::Return(None))
                } else {
                    self.expr().map(ast::Stmt::Return)
                }
            }
            Token::Keyword(Keyword::Break) => {
                self.scanner.get_next()?;
                if self.scanner.peek_next()? == Token::Delimiter(Delimiter::Semicolon) {
                    self.scanner.get_next()?;
                    Ok(ast::Stmt::Break(None))
                } else {
                    self.expr().map(ast::Stmt::Break)
                }
            }
            _ => {
                let expr = self.expr()?;
                match self.scanner.get_next()? {
                    Token::Assign(assigner) => {
                        let lcall = self.lcall(expr)?;
                        let expr = self.expr()?;
                        self.consume(Token::Delimiter(Delimiter::Semicolon))?;
                        Ok(ast::Stmt::Assignment(ast::Assignment {
                            lcall,
                            assigner,
                            expr,
                        }))
                    }
                    Token::Delimiter(Delimiter::Semicolon) => Ok(ast::Stmt::Expr(expr)),
                    token => return Err(Error::UnexpectedToken(token)),
                }
            }
        }
    }

    fn ident(&mut self) -> Result<IntStr> {
        match self.scanner.get_next()? {
            Token::Ident(ident) => Ok(ident),
            token => Err(Error::UnexpectedToken(token)),
        }
    }

    fn consume(&mut self, expected: Token) -> Result<()> {
        let token = self.scanner.get_next()?;
        if token == expected {
            Ok(())
        } else {
            Err(Error::UnexpectedToken(token))
        }
    }
}

impl Default for Parser {
    fn default() -> Parser {
        Parser::new()
    }
}
