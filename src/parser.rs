use crate::ast;
use crate::error::{Error, Result};
use crate::scanner::Scanner;
use crate::str_interner::IntStr;
use crate::token::{Assign, Delimiter, Keyword, Token};
use std::collections::HashMap;

pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse(scanner: Scanner) -> Result<ast::Program> {
        ParseState { scanner }.program()
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
                    if methods.contains_key(&fun.ident) {
                        return Err(Error::MethodDefinedTwice(fun.ident));
                    }
                    methods.insert(fun.ident, fun);
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
                    self.expr().map(|expr| ast::Stmt::Return(Some(expr)))
                }
            }
            Token::Keyword(Keyword::Break) => {
                self.scanner.get_next()?;
                if self.scanner.peek_next()? == Token::Delimiter(Delimiter::Semicolon) {
                    self.scanner.get_next()?;
                    Ok(ast::Stmt::Break(None))
                } else {
                    self.expr().map(|expr| ast::Stmt::Break(Some(expr)))
                }
            }
            _ => match self.assignment_or_expr()? {
                Either::A(assignment) => Ok(ast::Stmt::Assignment(assignment)),
                Either::B(expr) => {
                    self.consume(Token::Delimiter(Delimiter::Semicolon))?;
                    Ok(ast::Stmt::Expr(expr))
                }
            },
        }
    }

    fn assignment_or_expr(&mut self) -> Result<Either<ast::Assignment, ast::Expr>> {
        let expr = self.expr()?;
        match self.scanner.get_next()? {
            Token::Assign(assigner) => {
                let lcall = Self::lcall(expr)?;
                let expr = self.expr()?;
                self.consume(Token::Delimiter(Delimiter::Semicolon))?;
                Ok(Either::A(ast::Assignment {
                    lcall,
                    assigner,
                    expr,
                }))
            }
            token => {
                self.scanner.putback(token);
                Ok(Either::B(expr))
            }
        }
    }

    fn lcall(expr: ast::Expr) -> Result<ast::LCall> {
        let call = match expr {
            ast::Expr {
                logic_or:
                    ast::LogicOr::Next(ast::LogicAnd::Next(ast::Cmp::Next(ast::BitOr::Next(
                        ast::BitXor::Next(ast::BitAnd::Next(ast::Shift::Next(ast::Term::Next(
                            ast::Factor::Next(ast::Unary::Next(call)),
                        )))),
                    )))),
            } => call,
            _ => return Err(Error::UnassignableExpression),
        };

        let head = match call.head {
            ast::Primary::Ident(ident) => ast::LCallHead::Ident(ident),
            ast::Primary::SelfKw => ast::LCallHead::SelfKw,
            _ => return Err(Error::UnassignableExpression),
        };

        let tail = call
            .tail
            .into_iter()
            .map(|part| match part {
                ast::CallPart::Dot(ident) => Ok(ast::LCallPart::Dot(ident)),
                ast::CallPart::Brkts(expr) => Ok(ast::LCallPart::Brkts(expr)),
                _ => Err(Error::UnassignableExpression),
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(ast::LCall { head, tail })
    }

    fn expr(&mut self) -> Result<ast::Expr> {
        todo!()
    }

    fn block(&mut self) -> Result<ast::Block> {
        self.consume(Token::Delimiter(Delimiter::OpenCurly))?;

        let mut decls = Vec::new();

        loop {
            let token = self.scanner.get_next()?;
            match token {
                Token::Keyword(Keyword::Struct)
                | Token::Keyword(Keyword::Fn)
                | Token::Keyword(Keyword::Let)
                | Token::Keyword(Keyword::Return)
                | Token::Keyword(Keyword::Break) => {
                    self.scanner.putback(token);
                    decls.push(self.decl()?);
                }
                Token::Delimiter(Delimiter::CloseCurly) => {
                    return Ok(ast::Block { decls, expr: None });
                }
                token => {
                    self.scanner.putback(token);

                    match self.assignment_or_expr()? {
                        Either::A(assignment) => {
                            decls.push(ast::Decl::Stmt(ast::Stmt::Assignment(assignment)));
                        }
                        Either::B(expr) => {
                            self.consume(Token::Delimiter(Delimiter::CloseCurly))?;
                            return Ok(ast::Block {
                                decls,
                                expr: Some(Box::new(expr)),
                            });
                        }
                    }
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

enum Either<A, B> {
    A(A),
    B(B),
}
