// ============================================================
// SDT (Syntax-Directed Translation) — TINY 语言的语义分析与中间代码生成
//
// 本实验采用两遍策略：
//   第 1 遍：parser.rs — 语法分析 + 构造 AST（语法制导的语义动作）
//   第 2 遍：codegen.rs — 遍历 AST 生成 TM 中间代码（语法制导翻译）
//
// ── 产生式与语义动作（SDD / 属性文法）───────────────────
//
// 说明：下文对 TINY 语言的每一条上下文无关文法产生式，依次给出
// （a）"产生式类型"的中文说明——这条规则在语言中扮演什么角色；
// （b）"产生式含义"的中文说明——这条规则表达了什么样的语法结构；
// （c）语法制导语义动作——.ast（构造 AST）与 .code（中间代码）。
//
// （1）program → stmt-seq
//      类型：程序（起始产生式）
//      含义：一个 TINY 程序由若干条语句按顺序组成。文法中唯一不以非终结符
//            结尾的产生式，表示编译的入口点。
//      program.ast = Program(stmt-seq.stmts)
//      program.code = (∀ s ∈ stmt-seq.stmts) gen_stmt(s) || 'HALT'
//
// （2）stmt-seq → stmt₁ ; stmt₂ ; … ; stmtₙ
//      类型：语句序列
//      含义：用分号分隔的一条或多条语句的序列。用于表示 if 的 then/else 体、
//            repeat 的循环体以及程序的顶层语句列表。
//      stmt-seq.stmts = [stmt₁.ast, stmt₂.ast, …, stmtₙ.ast]
//
// （3）stmt → if-stmt | repeat-stmt | read-stmt | write-stmt | assign-stmt
//      类型：语句（综合选择产生式）
//      含义：TINY 语言的五种基本语句——条件分支、循环、读入、写出、赋值。
//            语法分析器根据 lookahead token 选择对应子规则。
//      stmt.ast = dispatch(lookahead)  // 递归下降分发
//
// （4）if-stmt → if cond = expr then S₁ = stmt-seq [ else S₂ = stmt-seq ] end
//      类型：if-then-else 条件语句
//      含义：若条件表达式 cond 为真（非零），则执行 then 分支 S₁；
//            若为假且存在 else 分支，则执行 S₂；以 end 关键字结束。
//      if-stmt.ast = Node(If, cond.ast, S₁.stmts, S₂?.stmts)
//      if-stmt.code =
//          cond.code
//          || JEQ AC, L_else_or_end   // false → 跳过 then
//          || S₁.code
//          || (S₂ ? LDA PC, L_end : ε)
//          || (S₂ ? L_else: S₂.code : ε)
//          || L_end:
//
// （5）repeat-stmt → repeat S = stmt-seq until cond = expr
//      类型：repeat-until 循环语句
//      含义：先执行循环体 S，然后计算条件 cond；若 cond 为假（零），
//            则重复执行循环体；否则退出循环。相当于 do-while 的语义。
//      repeat-stmt.ast = Node(Repeat, S.stmts, cond.ast)
//      repeat-stmt.code =
//          L_start: S.code || cond.code || JEQ AC, L_start
//
// （6）read-stmt → read id
//      类型：read 读入语句
//      含义：从标准输入读取一个整数值，存入变量 id 中。
//      read-stmt.ast = Node(Read, id.name)
//      read-stmt.code = IN AC,0,0 || ST AC, lookup(id.name)(GP)
//
// （7）write-stmt → write E = expr
//      类型：write 写出语句
//      含义：计算表达式 E 的值，并将结果输出到标准输出。
//      write-stmt.ast = Node(Write, E.ast)
//      write-stmt.code = E.code || OUT AC,0,0
//
// （8）assign-stmt → id := E = expr
//      类型：赋值语句
//      含义：计算右部表达式 E 的值，并将结果存入变量 id。
//      assign-stmt.ast = Node(Assign, id.name, E.ast)
//      assign-stmt.code = E.code || ST AC, lookup(id.name)(GP)
//
// （9）expr → E₁ = simple-expr [ relop E₂ = simple-expr ]
//      类型：比较表达式
//      含义：对两个简单表达式 E₁ 和 E₂ 做关系比较（< 或 =），
//            结果为布尔值（1 = true, 0 = false）。若无比较运算符，
//            则退化为简单表达式。
//      expr.ast = (relop) ? Node(Binary, relop, E₁.ast, E₂.ast) : E₁.ast
//      expr.code =
//          (relop) ? E₁.code || push(AC) || E₂.code || AC₁←pop || SUB AC,AC₁,AC
//                    || (JLT/JEQ) AC,2(PC) || LDC AC,0 || LDA PC,1(PC) || LDC AC,1
//                  : E₁.code
//
// （10）simple-expr → T₁ = term { addop ∈ {+,-} T₂ = term }
//       类型：简单表达式（加减表达式）
//       含义：由一个或多个 term 通过加法运算符 + 或减法运算符 - 左结合
//             连接而成。用于表达加减运算。
//       simple-expr.ast = left-fold(Node(Binary, addop, acc.ast, T₂.ast), init=T₁)
//       simple-expr.code =
//           T₁.code || (∀ T₂: push(AC) || T₂.code || AC₁←pop || (ADD/SUB) AC,AC₁,AC)
//
// （11）term → F₁ = factor { mulop ∈ {*,/} F₂ = factor }
//       类型：项（乘除表达式）
//       含义：由一个或多个 factor 通过乘法运算符 * 或除法运算符 / 左结合
//             连接而成。用于表达乘除运算，优先级高于加减。
//       term.ast = left-fold(Node(Binary, mulop, acc.ast, F₂.ast), init=F₁)
//       term.code =
//           F₁.code || (∀ F₂: push(AC) || F₂.code || AC₁←pop || (MUL/DIV) AC,AC₁,AC)
//
// （12）factor → ( expr ) | num | id
//       类型：因子（基本表达式单位）
//       含义：表达式的最小不可分割单元——可以是括号括起的子表达式、
//             整数字面量 num、或变量引用 id。优先级最高。
//       factor.ast = Node(Num, v) | Node(Id, name) | expr.ast
//       factor.code = LDC AC,v,0 | LD AC, lookup(name)(GP) | expr.code
//
// ── 符号表（语义分析） ─────────────────────────────────
//
// 在第 2 遍遍历 AST（codegen.rs）时维护：
//   symtab : HashMap<id.name, offset>
//   lookup(id.name) ≡ if symtab.contains(id.name) then symtab[id.name]
//                      else symtab.insert(id.name, next_global++)  // 首次遇到自动分配
//
// ============================================================

#![allow(dead_code)]

use super::ast::{BinOp, Expr, Program, Stmt};
use super::error::CompileError;
use super::scanner::{TinyToken, TinyTokenKind};

/// 对 token 序列做语法分析，返回 AST。
/// 这是 SDT 的第 1 遍：语法制导构造抽象语法树。
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