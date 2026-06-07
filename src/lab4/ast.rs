#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    If {
        cond: Expr,
        then_part: Vec<Stmt>,
        else_part: Option<Vec<Stmt>>,
    },
    Repeat {
        body: Vec<Stmt>,
        until: Expr,
    },
    Assign {
        name: String,
        expr: Expr,
    },
    Read {
        name: String,
    },
    Write {
        expr: Expr,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(i64),
    Id(String),
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Plus,
    Minus,
    Times,
    Over,
    Lt,
    Eq,
}