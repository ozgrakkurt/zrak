use crate::ast;
use crate::error::{Error, Result};
use crate::scanner::Scanner;
use crate::str_interner::IntStr;
use crate::token::{Assign, Delimiter, Keyword, Literal, Operator, TermOp, Token};
use std::collections::HashMap;

pub struct Parser<'a> {
    scanner: Scanner<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(scanner: Scanner<'a>) -> Parser<'a> {
        Parser { scanner }
    }

    pub fn program(&mut self) -> Result<ast::Program> {
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
        self.logic_or().map(|logic_or| ast::Expr { logic_or })
    }

    fn logic_or(&mut self) -> Result<ast::LogicOr> {
        let left = self.logic_and()?;
        let logic_or = match self.scanner.get_next()? {
            Token::Operator(Operator::LogicOr) => {
                ast::LogicOr::Current(left, Box::new(self.logic_or()?))
            }
            token => {
                self.scanner.putback(token);
                ast::LogicOr::Next(left)
            }
        };

        Ok(logic_or)
    }

    fn logic_and(&mut self) -> Result<ast::LogicAnd> {
        let left = self.cmp()?;
        let logic_and = match self.scanner.get_next()? {
            Token::Operator(Operator::LogicAnd) => {
                ast::LogicAnd::Current(left, Box::new(self.logic_and()?))
            }
            token => {
                self.scanner.putback(token);
                ast::LogicAnd::Next(left)
            }
        };

        Ok(logic_and)
    }

    fn cmp(&mut self) -> Result<ast::Cmp> {
        let left = self.bit_or()?;
        let cmp = match self.scanner.get_next()? {
            Token::Operator(Operator::Cmp(op)) => ast::Cmp::Current {
                left,
                op,
                cmp: Box::new(self.cmp()?),
            },
            token => {
                self.scanner.putback(token);
                ast::Cmp::Next(left)
            }
        };

        Ok(cmp)
    }

    fn bit_or(&mut self) -> Result<ast::BitOr> {
        let left = self.bit_xor()?;
        let bit_or = match self.scanner.get_next()? {
            Token::Operator(Operator::BitOr) => ast::BitOr::Current(left, Box::new(self.bit_or()?)),
            token => {
                self.scanner.putback(token);
                ast::BitOr::Next(left)
            }
        };

        Ok(bit_or)
    }

    fn bit_xor(&mut self) -> Result<ast::BitXor> {
        let left = self.bit_and()?;
        let bit_xor = match self.scanner.get_next()? {
            Token::Operator(Operator::BitXor) => {
                ast::BitXor::Current(left, Box::new(self.bit_xor()?))
            }
            token => {
                self.scanner.putback(token);
                ast::BitXor::Next(left)
            }
        };

        Ok(bit_xor)
    }

    fn bit_and(&mut self) -> Result<ast::BitAnd> {
        let left = self.shift()?;
        let bit_and = match self.scanner.get_next()? {
            Token::Operator(Operator::BitAnd) => {
                ast::BitAnd::Current(left, Box::new(self.bit_and()?))
            }
            token => {
                self.scanner.putback(token);
                ast::BitAnd::Next(left)
            }
        };

        Ok(bit_and)
    }

    fn shift(&mut self) -> Result<ast::Shift> {
        let left = self.term()?;
        let shift = match self.scanner.get_next()? {
            Token::Operator(Operator::Shift(op)) => ast::Shift::Current {
                left,
                op,
                shift: Box::new(self.shift()?),
            },
            token => {
                self.scanner.putback(token);
                ast::Shift::Next(left)
            }
        };

        Ok(shift)
    }

    fn term(&mut self) -> Result<ast::Term> {
        let left = self.factor()?;
        let term = match self.scanner.get_next()? {
            Token::Operator(Operator::Term(op)) => ast::Term::Current {
                left,
                op,
                term: Box::new(self.term()?),
            },
            token => {
                self.scanner.putback(token);
                ast::Term::Next(left)
            }
        };

        Ok(term)
    }

    fn factor(&mut self) -> Result<ast::Factor> {
        let left = self.unary()?;
        let term = match self.scanner.get_next()? {
            Token::Operator(Operator::Factor(op)) => ast::Factor::Current {
                left,
                op,
                factor: Box::new(self.factor()?),
            },
            token => {
                self.scanner.putback(token);
                ast::Factor::Next(left)
            }
        };

        Ok(term)
    }

    fn unary(&mut self) -> Result<ast::Unary> {
        let op = match self.scanner.get_next()? {
            Token::Operator(Operator::Not) => ast::UnaryOp::Not,
            Token::Operator(Operator::Term(TermOp::Sub)) => ast::UnaryOp::Negate,
            token => {
                self.scanner.putback(token);
                return Ok(ast::Unary::Next(self.call()?));
            }
        };

        Ok(ast::Unary::Current {
            op,
            unary: Box::new(self.unary()?),
        })
    }

    fn call(&mut self) -> Result<ast::Call> {
        let head = self.primary()?;
        let mut tail = Vec::new();

        loop {
            match self.scanner.get_next()? {
                Token::Delimiter(Delimiter::Dot) => {
                    let ident = self.ident()?;
                    tail.push(ast::CallPart::Dot(ident))
                }
                Token::Delimiter(Delimiter::OpenBrkt) => {
                    let expr = self.expr()?;
                    self.consume(Token::Delimiter(Delimiter::CloseBrkt))?;
                    tail.push(ast::CallPart::Brkts(Box::new(expr)))
                }
                Token::Delimiter(Delimiter::OpenPrnth) => {
                    let args = self.expr_list(Token::Delimiter(Delimiter::ClosePrnth))?;
                    tail.push(ast::CallPart::FunCall(args))
                }
                Token::Operator(Operator::QMark) => tail.push(ast::CallPart::QMark),
                token => {
                    self.scanner.putback(token);
                    break;
                }
            }
        }

        Ok(ast::Call { head, tail })
    }

    fn expr_list(&mut self, sentinel: Token) -> Result<Vec<ast::Expr>> {
        let mut expr_list = Vec::new();

        loop {
            match self.scanner.get_next()? {
                t if t == sentinel => {
                    break;
                }
                token => {
                    self.scanner.putback(token);
                    expr_list.push(self.expr()?);
                    match self.scanner.get_next()? {
                        Token::Delimiter(Delimiter::Comma) => (),
                        t if t == sentinel => break,
                        token => return Err(Error::UnexpectedToken(token)),
                    }
                }
            }
        }

        Ok(expr_list)
    }

    fn primary(&mut self) -> Result<ast::Primary> {
        let primary = match self.scanner.get_next()? {
            Token::Keyword(Keyword::SelfKw) => ast::Primary::SelfKw,
            Token::Delimiter(Delimiter::OpenPrnth) => {
                let expr = self.expr()?;
                self.consume(Token::Delimiter(Delimiter::ClosePrnth))?;
                ast::Primary::Prnth(Box::new(expr))
            }
            Token::Ident(ident) => ast::Primary::Ident(ident),
            Token::Keyword(Keyword::For) => ast::Primary::For(self.for_loop()?),
            Token::Keyword(Keyword::While) => ast::Primary::While(self.while_loop()?),
            Token::Keyword(Keyword::Loop) => ast::Primary::Loop(self.loop_loop()?),
            Token::Keyword(Keyword::If) => ast::Primary::If(self.if_expr()?),
            Token::Operator(Operator::BitOr) => ast::Primary::Closure(self.closure()?),
            Token::Delimiter(Delimiter::OpenCurly) => {
                self.scanner.putback(Token::Delimiter(Delimiter::OpenCurly));
                ast::Primary::Block(self.block()?)
            }
            token => {
                self.scanner.putback(token);
                ast::Primary::Literal(self.literal()?)
            }
        };

        Ok(primary)
    }

    fn for_loop(&mut self) -> Result<ast::For> {
        let ident = self.ident()?;
        self.consume(Token::Keyword(Keyword::In))?;
        let expr = self.expr()?;
        let block = self.block()?;

        Ok(ast::For {
            ident,
            expr: Box::new(expr),
            block,
        })
    }

    fn while_loop(&mut self) -> Result<ast::While> {
        let cond = self.expr()?;
        let block = self.block()?;

        Ok(ast::While {
            cond: Box::new(cond),
            block,
        })
    }

    fn loop_loop(&mut self) -> Result<ast::Loop> {
        let block = self.block()?;

        Ok(ast::Loop { block })
    }

    fn if_expr(&mut self) -> Result<ast::If> {
        let cond = self.expr()?;
        let block = self.block()?;

        let els = match self.scanner.get_next()? {
            Token::Keyword(Keyword::Else) => Some(self.els()?),
            token => {
                self.scanner.putback(token);
                None
            }
        };

        Ok(ast::If {
            cond: Box::new(cond),
            block,
            els,
        })
    }

    fn els(&mut self) -> Result<ast::Else> {
        let els = match self.scanner.get_next()? {
            Token::Keyword(Keyword::If) => ast::Else::If(Box::new(self.if_expr()?)),
            token => {
                self.scanner.putback(token);
                ast::Else::Block(self.block()?)
            }
        };

        Ok(els)
    }

    fn closure(&mut self) -> Result<ast::Closure> {
        let params = self.params()?;
        self.consume(Token::Operator(Operator::BitOr))?;
        let block = self.block()?;
        Ok(ast::Closure { params, block })
    }

    fn literal(&mut self) -> Result<ast::Literal> {
        let lit = match self.scanner.get_next()? {
            Token::Literal(Literal::Bool(b)) => ast::Literal::Bool(b),
            Token::Literal(Literal::Null) => ast::Literal::Null,
            Token::Literal(Literal::Int(i)) => ast::Literal::Int(i),
            Token::Literal(Literal::Float(f)) => ast::Literal::Float(f),
            Token::Literal(Literal::Char(c)) => ast::Literal::Char(c),
            Token::Literal(Literal::Str(s)) => ast::Literal::Str(s),
            Token::Keyword(Keyword::New) => ast::Literal::Struct(self.struct_lit()?),
            Token::Keyword(Keyword::Map) => ast::Literal::Map(self.map_lit()?),
            Token::Delimiter(Delimiter::OpenBrkt) => ast::Literal::Array(self.array_lit()?),
            token => return Err(Error::UnexpectedToken(token)),
        };

        Ok(lit)
    }

    fn struct_lit(&mut self) -> Result<ast::StructLit> {
        let ident = self.ident()?;
        self.consume(Token::Delimiter(Delimiter::OpenCurly))?;
        let mut fields = Vec::new();
        loop {
            match self.scanner.get_next()? {
                Token::Ident(field_name) => {
                    self.consume(Token::Delimiter(Delimiter::Colon))?;
                    let field_value = self.expr()?;
                    fields.push((field_name, field_value));
                    match self.scanner.get_next()? {
                        Token::Delimiter(Delimiter::Comma) => (),
                        Token::Delimiter(Delimiter::CloseCurly) => break,
                        token => return Err(Error::UnexpectedToken(token)),
                    }
                }
                Token::Delimiter(Delimiter::CloseCurly) => break,
                token => return Err(Error::UnexpectedToken(token)),
            }
        }

        Ok(ast::StructLit { ident, fields })
    }

    fn map_lit(&mut self) -> Result<ast::MapLit> {
        self.consume(Token::Delimiter(Delimiter::OpenCurly))?;

        let mut fields = Vec::new();
        loop {
            match self.scanner.get_next()? {
                Token::Delimiter(Delimiter::CloseCurly) => break,
                token => {
                    self.scanner.putback(token);
                    let key = self.expr()?;
                    self.consume(Token::Delimiter(Delimiter::Colon))?;
                    let value = self.expr()?;
                    fields.push((key, value));
                    match self.scanner.get_next()? {
                        Token::Delimiter(Delimiter::Comma) => (),
                        Token::Delimiter(Delimiter::CloseCurly) => break,
                        token => return Err(Error::UnexpectedToken(token)),
                    }
                }
            }
        }

        Ok(ast::MapLit { fields })
    }

    fn array_lit(&mut self) -> Result<ast::ArrayLit> {
        let mut elems = Vec::new();
        loop {
            match self.scanner.get_next()? {
                Token::Delimiter(Delimiter::CloseBrkt) => break,
                token => {
                    self.scanner.putback(token);
                    let expr = self.expr()?;
                    elems.push(expr);
                    match self.scanner.get_next()? {
                        Token::Delimiter(Delimiter::Comma) => (),
                        Token::Delimiter(Delimiter::CloseBrkt) => break,
                        token => return Err(Error::UnexpectedToken(token)),
                    }
                }
            }
        }

        Ok(ast::ArrayLit { elems })
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
                        Either::B(expr) => match self.scanner.get_next()? {
                            Token::Delimiter(Delimiter::CloseCurly) => {
                                return Ok(ast::Block {
                                    decls,
                                    expr: Some(Box::new(expr)),
                                });
                            }
                            Token::Delimiter(Delimiter::Semicolon) => {
                                decls.push(ast::Decl::Stmt(ast::Stmt::Expr(expr)));
                            }
                            token => return Err(Error::UnexpectedToken(token)),
                        },
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

enum Either<A, B> {
    A(A),
    B(B),
}
