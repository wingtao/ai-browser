use thiserror::Error;

use super::{
    ast::{AssignTarget, BinaryOp, Expr, Program, Stmt, UnaryOp, VarKind},
    lexer::{lex, Token, TokenKind},
};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("unexpected token at {pos}")]
    UnexpectedToken { pos: usize },
    #[error("expected token `{expected}` at {pos}")]
    ExpectedToken { expected: &'static str, pos: usize },
    #[error("{0}")]
    Lex(#[from] super::lexer::LexError),
}

pub fn parse_program(code: &str) -> Result<Program, ParseError> {
    let tokens = lex(code)?;
    let mut parser = Parser { tokens, idx: 0 };
    parser.program()
}

struct Parser {
    tokens: Vec<Token>,
    idx: usize,
}

impl Parser {
    fn program(&mut self) -> Result<Program, ParseError> {
        let mut body = vec![];
        while !self.is(TokenKind::Eof) {
            body.push(self.statement()?);
        }
        Ok(Program { body })
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        match self.peek_kind() {
            TokenKind::Let => self.var_decl(VarKind::Let),
            TokenKind::Const => self.var_decl(VarKind::Const),
            TokenKind::Var => self.var_decl(VarKind::Var),
            TokenKind::Function => self.function_decl(),
            TokenKind::Return => self.return_stmt(),
            TokenKind::If => self.if_stmt(),
            TokenKind::While => self.while_stmt(),
            TokenKind::LBrace => self.block_stmt(),
            _ => {
                let expr = self.expression()?;
                self.consume_if(TokenKind::Semicolon);
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn var_decl(&mut self, kind: VarKind) -> Result<Stmt, ParseError> {
        self.advance();
        let name = self.expect_ident()?;
        let init = if self.consume_if(TokenKind::Assign) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume_if(TokenKind::Semicolon);
        Ok(Stmt::VarDecl { kind, name, init })
    }

    fn function_decl(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenKind::Function, "function")?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::LParen, "(")?;
        let mut params = vec![];
        if !self.is(TokenKind::RParen) {
            loop {
                params.push(self.expect_ident()?);
                if !self.consume_if(TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect(TokenKind::RParen, ")")?;
        let body = match self.block_stmt()? {
            Stmt::Block(stmts) => stmts,
            _ => unreachable!(),
        };
        Ok(Stmt::FunctionDecl { name, params, body })
    }

    fn return_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenKind::Return, "return")?;
        let expr = if self.is(TokenKind::Semicolon) || self.is(TokenKind::RBrace) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume_if(TokenKind::Semicolon);
        Ok(Stmt::Return(expr))
    }

    fn if_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenKind::If, "if")?;
        self.expect(TokenKind::LParen, "(")?;
        let test = self.expression()?;
        self.expect(TokenKind::RParen, ")")?;
        let consequent = Box::new(self.statement()?);
        let alternate = if self.consume_if(TokenKind::Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::If {
            test,
            consequent,
            alternate,
        })
    }

    fn while_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenKind::While, "while")?;
        self.expect(TokenKind::LParen, "(")?;
        let test = self.expression()?;
        self.expect(TokenKind::RParen, ")")?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While { test, body })
    }

    fn block_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenKind::LBrace, "{")?;
        let mut stmts = vec![];
        while !self.is(TokenKind::RBrace) && !self.is(TokenKind::Eof) {
            stmts.push(self.statement()?);
        }
        self.expect(TokenKind::RBrace, "}")?;
        Ok(Stmt::Block(stmts))
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.logical_or()?;
        if self.consume_if(TokenKind::Assign) {
            let value = self.assignment()?;
            let target = match expr {
                Expr::Identifier(name) => AssignTarget::Identifier(name),
                Expr::Member { object, property } => AssignTarget::Member { object, property },
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        pos: self.current().pos,
                    })
                }
            };
            Ok(Expr::Assignment {
                target,
                value: Box::new(value),
            })
        } else {
            Ok(expr)
        }
    }

    fn logical_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.logical_and()?;
        while self.consume_if(TokenKind::OrOr) {
            let right = self.logical_and()?;
            expr = Expr::Binary {
                op: BinaryOp::Or,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn logical_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;
        while self.consume_if(TokenKind::AndAnd) {
            let right = self.equality()?;
            expr = Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        loop {
            if self.consume_if(TokenKind::EqEq) {
                let right = self.comparison()?;
                expr = Expr::Binary {
                    op: BinaryOp::Eq,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            } else if self.consume_if(TokenKind::NotEq) {
                let right = self.comparison()?;
                expr = Expr::Binary {
                    op: BinaryOp::NotEq,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        loop {
            let op = if self.consume_if(TokenKind::Gt) {
                Some(BinaryOp::Gt)
            } else if self.consume_if(TokenKind::Gte) {
                Some(BinaryOp::Gte)
            } else if self.consume_if(TokenKind::Lt) {
                Some(BinaryOp::Lt)
            } else if self.consume_if(TokenKind::Lte) {
                Some(BinaryOp::Lte)
            } else {
                None
            };

            let Some(op) = op else {
                break;
            };
            let right = self.term()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        loop {
            let op = if self.consume_if(TokenKind::Plus) {
                Some(BinaryOp::Add)
            } else if self.consume_if(TokenKind::Minus) {
                Some(BinaryOp::Sub)
            } else {
                None
            };
            let Some(op) = op else { break };
            let right = self.factor()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        loop {
            let op = if self.consume_if(TokenKind::Star) {
                Some(BinaryOp::Mul)
            } else if self.consume_if(TokenKind::Slash) {
                Some(BinaryOp::Div)
            } else {
                None
            };
            let Some(op) = op else { break };
            let right = self.unary()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.consume_if(TokenKind::Bang) {
            let expr = self.unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(expr),
            });
        }
        if self.consume_if(TokenKind::Minus) {
            let expr = self.unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(expr),
            });
        }
        self.call_member()
    }

    fn call_member(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;
        loop {
            if self.consume_if(TokenKind::Dot) {
                let prop = self.expect_ident()?;
                expr = Expr::Member {
                    object: Box::new(expr),
                    property: prop,
                };
            } else if self.consume_if(TokenKind::LParen) {
                let mut args = vec![];
                if !self.is(TokenKind::RParen) {
                    loop {
                        args.push(self.expression()?);
                        if !self.consume_if(TokenKind::Comma) {
                            break;
                        }
                    }
                }
                self.expect(TokenKind::RParen, ")")?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.current().clone();
        self.advance();
        match token.kind {
            TokenKind::Number(v) => Ok(Expr::Number(v)),
            TokenKind::String(v) => Ok(Expr::String(v)),
            TokenKind::True => Ok(Expr::Bool(true)),
            TokenKind::False => Ok(Expr::Bool(false)),
            TokenKind::Null => Ok(Expr::Null),
            TokenKind::Undefined => Ok(Expr::Undefined),
            TokenKind::Identifier(v) => Ok(Expr::Identifier(v)),
            TokenKind::LParen => {
                let e = self.expression()?;
                self.expect(TokenKind::RParen, ")")?;
                Ok(e)
            }
            TokenKind::LBrace => self.object_literal(),
            _ => Err(ParseError::UnexpectedToken { pos: token.pos }),
        }
    }

    fn object_literal(&mut self) -> Result<Expr, ParseError> {
        let mut props = vec![];
        while !self.is(TokenKind::RBrace) && !self.is(TokenKind::Eof) {
            let key = match self.current().kind.clone() {
                TokenKind::Identifier(k) | TokenKind::String(k) => {
                    self.advance();
                    k
                }
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        pos: self.current().pos,
                    })
                }
            };
            self.expect(TokenKind::Colon, ":")?;
            let value = self.expression()?;
            props.push((key, value));
            if !self.consume_if(TokenKind::Comma) {
                break;
            }
        }
        self.expect(TokenKind::RBrace, "}")?;
        Ok(Expr::ObjectLiteral(props))
    }

    fn current(&self) -> &Token {
        &self.tokens[self.idx]
    }

    fn peek_kind(&self) -> TokenKind {
        self.current().kind.clone()
    }

    fn is(&self, kind: TokenKind) -> bool {
        self.current().kind == kind
    }

    fn advance(&mut self) {
        if self.idx < self.tokens.len() - 1 {
            self.idx += 1;
        }
    }

    fn expect(&mut self, kind: TokenKind, expected: &'static str) -> Result<(), ParseError> {
        if self.current().kind == kind {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::ExpectedToken {
                expected,
                pos: self.current().pos,
            })
        }
    }

    fn consume_if(&mut self, kind: TokenKind) -> bool {
        if self.current().kind == kind {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect_ident(&mut self) -> Result<String, ParseError> {
        match self.current().kind.clone() {
            TokenKind::Identifier(name) => {
                self.advance();
                Ok(name)
            }
            _ => Err(ParseError::ExpectedToken {
                expected: "identifier",
                pos: self.current().pos,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fn_and_if() {
        let code = r#"
            function add(a,b){ return a+b; }
            let x = add(1,2);
            if (x > 2) { x = x + 1; }
        "#;
        let program = parse_program(code).unwrap();
        assert_eq!(program.body.len(), 3);
    }
}
