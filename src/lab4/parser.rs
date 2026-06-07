#![allow(dead_code)]

use super::ast::{BinOp, Expr, Program, Stmt};
use super::error::CompileError;
use super::scanner::{TinyToken, TinyTokenKind};

/// 对 token 序列做语法分析，返回 AST。
pub fn parse(tokens: &[TinyToken]) -> Result<Program, CompileError> {
    let mut p = Parser::new(tokens);
    let stmts = p.parse_stmt_seq()?;
    p.expect_simple(TinyTokenKind::Eof)?;
    Ok(Program { stmts })
}

struct Parser<'a> {
    tokens: &'a [TinyToken],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [TinyToken]) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &'a TinyToken {
        self.tokens
            .get(self.pos)
            .unwrap_or_else(|| self.tokens.last().expect("tokens 非空"))
    }

    fn advance(&mut self) -> &'a TinyToken {
        let tok = self.peek();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    /// 检查当前 token 是否为给定的简单类型（不含 data 的枚举变体）。
    fn check_simple(&self, kind: TinyTokenKind) -> bool {
        // 利用 matches! 宏只关心变体形状
        matches!(
            (&self.peek().kind, kind),
            (TinyTokenKind::If, TinyTokenKind::If)
                | (TinyTokenKind::Then, TinyTokenKind::Then)
                | (TinyTokenKind::Else, TinyTokenKind::Else)
                | (TinyTokenKind::End, TinyTokenKind::End)
                | (TinyTokenKind::Repeat, TinyTokenKind::Repeat)
                | (TinyTokenKind::Until, TinyTokenKind::Until)
                | (TinyTokenKind::Read, TinyTokenKind::Read)
                | (TinyTokenKind::Write, TinyTokenKind::Write)
                | (TinyTokenKind::Assign, TinyTokenKind::Assign)
                | (TinyTokenKind::Lt, TinyTokenKind::Lt)
                | (TinyTokenKind::Eq, TinyTokenKind::Eq)
                | (TinyTokenKind::Plus, TinyTokenKind::Plus)
                | (TinyTokenKind::Minus, TinyTokenKind::Minus)
                | (TinyTokenKind::Times, TinyTokenKind::Times)
                | (TinyTokenKind::Over, TinyTokenKind::Over)
                | (TinyTokenKind::LParen, TinyTokenKind::LParen)
                | (TinyTokenKind::RParen, TinyTokenKind::RParen)
                | (TinyTokenKind::Semi, TinyTokenKind::Semi)
                | (TinyTokenKind::Eof, TinyTokenKind::Eof)
        )
    }

    /// 消费指定的简单 token；不匹配则报错。
    fn expect_simple(&mut self, kind: TinyTokenKind) -> Result<(), CompileError> {
        if self.check_simple(kind.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(CompileError::Parse(format!(
                "期望 {:?}，但得到 {:?} (lexeme={})",
                kind,
                self.peek().kind,
                self.peek().lexeme
            )))
        }
    }

    // ── 语句序列 ────────────────────────────────────────

    /// stmt_seq → stmt { ; stmt }
    fn parse_stmt_seq(&mut self) -> Result<Vec<Stmt>, CompileError> {
        let mut stmts = Vec::new();
        stmts.push(self.parse_stmt()?);
        while self.check_simple(TinyTokenKind::Semi) {
            self.advance();
            // 在 end/else/until/eof 前停止（避免空语句）
            if matches!(
                self.peek().kind,
                TinyTokenKind::End
                    | TinyTokenKind::Else
                    | TinyTokenKind::Until
                    | TinyTokenKind::Eof
            ) {
                break;
            }
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }

    // ── 语句 ────────────────────────────────────────────

    fn parse_stmt(&mut self) -> Result<Stmt, CompileError> {
        match &self.peek().kind {
            TinyTokenKind::If => self.parse_if(),
            TinyTokenKind::Repeat => self.parse_repeat(),
            TinyTokenKind::Read => self.parse_read(),
            TinyTokenKind::Write => self.parse_write(),
            TinyTokenKind::Id(_) => self.parse_assign(),
            other => Err(CompileError::Parse(format!(
                "无法解析的语句起始 token: {:?} (lexeme={})",
                other,
                self.peek().lexeme
            ))),
        }
    }

    // if 表达式 then 语句序列 [ else 语句序列 ] end
    fn parse_if(&mut self) -> Result<Stmt, CompileError> {
        self.expect_simple(TinyTokenKind::If)?;
        let cond = self.parse_expr()?;
        self.expect_simple(TinyTokenKind::Then)?;
        let then_part = self.parse_stmt_seq()?;
        let else_part = if self.check_simple(TinyTokenKind::Else) {
            self.advance();
            Some(self.parse_stmt_seq()?)
        } else {
            None
        };
        self.expect_simple(TinyTokenKind::End)?;
        Ok(Stmt::If {
            cond,
            then_part,
            else_part,
        })
    }

    // repeat 语句序列 until 表达式
    fn parse_repeat(&mut self) -> Result<Stmt, CompileError> {
        self.expect_simple(TinyTokenKind::Repeat)?;
        let body = self.parse_stmt_seq()?;
        self.expect_simple(TinyTokenKind::Until)?;
        let until = self.parse_expr()?;
        Ok(Stmt::Repeat { body, until })
    }

    // read 标识符
    fn parse_read(&mut self) -> Result<Stmt, CompileError> {
        self.expect_simple(TinyTokenKind::Read)?;
        let name = match &self.peek().kind {
            TinyTokenKind::Id(s) => {
                let s = s.clone();
                self.advance();
                s
            }
            _ => {
                return Err(CompileError::Parse(format!(
                    "read 后应为标识符，但得到 {:?}",
                    self.peek().kind
                )))
            }
        };
        Ok(Stmt::Read { name })
    }

    // write 表达式
    fn parse_write(&mut self) -> Result<Stmt, CompileError> {
        self.expect_simple(TinyTokenKind::Write)?;
        let expr = self.parse_expr()?;
        Ok(Stmt::Write { expr })
    }

    // 标识符 := 表达式
    fn parse_assign(&mut self) -> Result<Stmt, CompileError> {
        let name = match &self.peek().kind {
            TinyTokenKind::Id(s) => {
                let s = s.clone();
                self.advance();
                s
            }
            _ => unreachable!(),
        };
        self.expect_simple(TinyTokenKind::Assign)?;
        let expr = self.parse_expr()?;
        Ok(Stmt::Assign { name, expr })
    }

    // ── 表达式 ──────────────────────────────────────────

    /// expr → simple_expr [ (< | =) simple_expr ]
    fn parse_expr(&mut self) -> Result<Expr, CompileError> {
        let mut left = self.parse_simple_expr()?;
        if self.check_simple(TinyTokenKind::Lt) || self.check_simple(TinyTokenKind::Eq) {
            let op = match &self.peek().kind {
                TinyTokenKind::Lt => BinOp::Lt,
                TinyTokenKind::Eq => BinOp::Eq,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_simple_expr()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// simple_expr → term { (+ | -) term }
    fn parse_simple_expr(&mut self) -> Result<Expr, CompileError> {
        let mut expr = self.parse_term()?;
        loop {
            let op = match &self.peek().kind {
                TinyTokenKind::Plus => Some(BinOp::Plus),
                TinyTokenKind::Minus => Some(BinOp::Minus),
                _ => None,
            };
            let Some(op) = op else { break };
            self.advance();
            let rhs = self.parse_term()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(rhs),
            };
        }
        Ok(expr)
    }

    /// term → factor { (* | /) factor }
    fn parse_term(&mut self) -> Result<Expr, CompileError> {
        let mut expr = self.parse_factor()?;
        loop {
            let op = match &self.peek().kind {
                TinyTokenKind::Times => Some(BinOp::Times),
                TinyTokenKind::Over => Some(BinOp::Over),
                _ => None,
            };
            let Some(op) = op else { break };
            self.advance();
            let rhs = self.parse_factor()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(rhs),
            };
        }
        Ok(expr)
    }

    /// factor → ( expr ) | num | id
    fn parse_factor(&mut self) -> Result<Expr, CompileError> {
        match &self.peek().kind {
            TinyTokenKind::Num(v) => {
                let v = *v;
                self.advance();
                Ok(Expr::Num(v))
            }
            TinyTokenKind::Id(s) => {
                let name = s.clone();
                self.advance();
                Ok(Expr::Id(name))
            }
            TinyTokenKind::LParen => {
                self.advance();
                let e = self.parse_expr()?;
                self.expect_simple(TinyTokenKind::RParen)?;
                Ok(e)
            }
            other => Err(CompileError::Parse(format!(
                "无法解析因子: {:?} (lexeme={})",
                other,
                self.peek().lexeme
            ))),
        }
    }
}