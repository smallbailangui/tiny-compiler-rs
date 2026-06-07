#![allow(dead_code)]

use std::collections::HashMap;

use super::ast::{BinOp, Expr, Program, Stmt};
use super::error::CompileError;

/// TM 虚拟机的寄存器索引常量。
const AC: i32 = 0;
const AC1: i32 = 1;
const GP: i32 = 5;
const MP: i32 = 6;
const PC: i32 = 7;

/// 对 AST 做中间代码生成，输出 TM 汇编文本。
pub fn generate_tm(program: &Program) -> Result<String, CompileError> {
    let mut generator = Generator::new();


    // 标准 TM 前置代码：
    //   LD  MP, 0(0)    从内存地址 0 读入程序最大地址（由用户或装载器填入）
    //   LDC GP, 0(0)    设置全局指针 GP = 0
    //   ST  AC, 0(0)    清空地址 0（方便后续 I/O）
    generator.emit_rm("LD", MP, 0, 0, "load maxaddress from location 0");
    generator.emit_rm("LDC", GP, 0, 0, "set GP = 0");
    generator.emit_rm("ST", AC, 0, 0, "clear location 0");

    for s in &program.stmts {
        generator.gen_stmt(s)?;
    }

    generator.emit_ro("HALT", 0, 0, 0, "done");

    Ok(generator.finish())
}

// ── 代码生成器状态 ────────────────────────────────────

struct Generator {
    emitter: Emitter,
    /// 变量名 → 全局偏移量映射
    symtab: HashMap<String, i32>,
    /// 下一个可用的全局变量偏移
    next_global: i32,
    /// 临时变量栈顶偏移（相对 MP，向下增长）
    tmp_offset: i32,
}

impl Generator {
    fn new() -> Self {
        Self {
            emitter: Emitter::new(),
            symtab: HashMap::new(),
            next_global: 0,
            tmp_offset: 0,
        }
    }

    fn finish(self) -> String {
        self.emitter.into_string()
    }

    /// 获取变量在 GP 区域的偏移。首次遇到时分配。
    fn var_offset(&mut self, name: &str) -> i32 {
        if let Some(&o) = self.symtab.get(name) {
            return o;
        }
        let o = self.next_global;
        self.next_global += 1;
        self.symtab.insert(name.to_string(), o);
        o
    }

    // ── 语句翻译 ───────────────────────────────────────

    fn gen_stmt(&mut self, stmt: &Stmt) -> Result<(), CompileError> {
        match stmt {
            Stmt::Read { name } => {
                let o = self.var_offset(name);
                self.emit_ro("IN", AC, 0, 0, "read integer");
                self.emit_rm("ST", AC, o, GP, &format!("store to {}", name));
            }
            Stmt::Write { expr } => {
                self.gen_expr(expr)?;
                self.emit_ro("OUT", AC, 0, 0, "write integer");
            }
            Stmt::Assign { name, expr } => {
                let o = self.var_offset(name);
                self.gen_expr(expr)?;
                self.emit_rm("ST", AC, o, GP, &format!("assign to {}", name));
            }
            Stmt::If {
                cond,
                then_part,
                else_part,
            } => self.gen_if(cond, then_part, else_part.as_deref())?,
            Stmt::Repeat { body, until } => self.gen_repeat(body, until)?,
        }
        Ok(())
    }

    /// 翻译 if 语句。
    ///
    /// 策略：
    ///   1. 计算条件表达式 → AC
    ///   2. 预留 false 跳转槽位
    ///   3. 生成 then 分支
    ///   4. 若有 else 分支：
    ///      a. 预留跳转到 end 的槽位
    ///      b. backpatch false → else 入口
    ///      c. 生成 else 分支
    ///      d. backpatch jmp → end
    ///   5. 若无 else：backpatch false → end
    fn gen_if(
        &mut self,
        cond: &Expr,
        then_part: &[Stmt],
        else_part: Option<&[Stmt]>,
    ) -> Result<(), CompileError> {
        self.gen_expr(cond)?;
        // 若 AC == 0（即条件为假），则跳过 then 分支
        let saved_jmp_false = self.emitter.emit_skip(1);

        for s in then_part {
            self.gen_stmt(s)?;
        }

        if let Some(else_stmts) = else_part {
            // then 结束后需要跳过 else 分支
            let saved_jmp_end = self.emitter.emit_skip(1);
            let else_start = self.emitter.emit_loc();

            // backpatch 假跳 → else 入口
            self.emitter.emit_backup(saved_jmp_false);
            self.emit_rm_abs("JEQ", AC, else_start, "if: jump to else");
            self.emitter.emit_restore();

            for s in else_stmts {
                self.gen_stmt(s)?;
            }
            let end = self.emitter.emit_loc();
            self.emitter.emit_backup(saved_jmp_end);
            self.emit_rm_abs("LDA", PC, end, "if: jump to end");
            self.emitter.emit_restore();
        } else {
            let end = self.emitter.emit_loc();
            self.emitter.emit_backup(saved_jmp_false);
            self.emit_rm_abs("JEQ", AC, end, "if: jump to end");
            self.emitter.emit_restore();
        }
        Ok(())
    }

    /// 翻译 repeat 语句。
    ///
    /// 策略：
    ///   1. 记录循环起始地址
    ///   2. 生成循环体
    ///   3. 计算 until 表达式 → AC
    ///   4. 若 AC == 0（条件仍为假），跳回循环起始处
    fn gen_repeat(&mut self, body: &[Stmt], until: &Expr) -> Result<(), CompileError> {
        let start = self.emitter.emit_loc();
        for s in body {
            self.gen_stmt(s)?;
        }
        self.gen_expr(until)?;
        self.emit_rm_abs("JEQ", AC, start, "repeat: loop back if false");
        Ok(())
    }

    // ── 表达式翻译 ─────────────────────────────────────

    fn gen_expr(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match expr {
            Expr::Num(v) => {
                self.emit_rm("LDC", AC, *v as i32, 0, "load const");
            }
            Expr::Id(name) => {
                let o = self.var_offset(name);
                self.emit_rm("LD", AC, o, GP, &format!("load id {}", name));
            }
            Expr::Binary { op, left, right } => {
                // 左操作数入栈（压到 MP 临时区）
                self.gen_expr(left)?;
                self.push_tmp("op: push left");
                // 右操作数
                self.gen_expr(right)?;
                // 弹出左操作数到 AC1
                self.pop_to_ac1("op: pop left");
                match op {
                    BinOp::Plus => self.emit_ro("ADD", AC, AC1, AC, "op +"),
                    BinOp::Minus => self.emit_ro("SUB", AC, AC1, AC, "op -"),
                    BinOp::Times => self.emit_ro("MUL", AC, AC1, AC, "op *"),
                    BinOp::Over => self.emit_ro("DIV", AC, AC1, AC, "op /"),
                    BinOp::Lt => self.gen_relop("JLT", "op <")?,
                    BinOp::Eq => self.gen_relop("JEQ", "op =")?,
                }
            }
        }
        Ok(())
    }

    /// 生成关系运算符的代码。
    ///
    /// 此时 AC = 右操作数，AC1 = 左操作数。
    /// 先计算 left - right，再根据条件跳转设置 AC 为 0 或 1。
    fn gen_relop(&mut self, jump_op: &str, cmt: &str) -> Result<(), CompileError> {
        // AC ← AC1 - AC = left - right
        self.emit_ro("SUB", AC, AC1, AC, "relop: left - right");

        // 若满足关系，跳转到加载 1
        self.emit_rm(jump_op, AC, 2, PC, cmt);
        // 不满足 → 加载 0
        self.emit_rm("LDC", AC, 0, 0, "relop: false");
        // 跳过加载 1
        self.emit_rm("LDA", PC, 1, PC, "relop: skip true");
        // 满足 → 加载 1
        self.emit_rm("LDC", AC, 1, 0, "relop: true");
        Ok(())
    }

    // ── 临时变量栈操作 ─────────────────────────────────

    fn push_tmp(&mut self, cmt: &str) {
        self.emit_rm("ST", AC, self.tmp_offset, MP, cmt);
        self.tmp_offset -= 1;
    }

    fn pop_to_ac1(&mut self, cmt: &str) {
        self.tmp_offset += 1;
        self.emit_rm("LD", AC1, self.tmp_offset, MP, cmt);
    }

    // ── 发射便捷方法 ───────────────────────────────────

    fn emit_comment(&mut self, comment: String) {
        self.emitter.emit_comment(comment)
    }

    fn emit_ro(&mut self, op: &str, r: i32, s: i32, t: i32, comment: &str) {
        self.emitter.emit_ro(op, r, s, t, comment)
    }

    fn emit_rm(&mut self, op: &str, r: i32, d: i32, s: i32, comment: &str) {
        self.emitter.emit_rm(op, r, d, s, comment)
    }

    fn emit_rm_abs(&mut self, op: &str, r: i32, a: i32, comment: &str) {
        self.emitter.emit_rm_abs(op, r, a, comment)
    }
}

// ── 发射器 ────────────────────────────────────────────

/// 低级指令发射器，支持 backpatching（回填）。
struct Emitter {
    header: Vec<String>,
    code: Vec<String>,
    emit_loc: i32,
    high_emit_loc: i32,
}

impl Emitter {
    fn new() -> Self {
        Self {
            header: Vec::new(),
            code: Vec::new(),
            emit_loc: 0,
            high_emit_loc: 0,
        }
    }

    fn emit_loc(&self) -> i32 {
        self.emit_loc
    }

    fn emit_comment(&mut self, comment: String) {
        self.header.push(format!("* {}", comment));
    }

    /// 发射一条 RO 格式指令（op r, s, t）。
    fn emit_ro(&mut self, op: &str, r: i32, s: i32, t: i32, comment: &str) {
        let line = format!(
            "{:>3}:  {:<5} {},{},{}\t* {}",
            self.emit_loc, op, r, s, t, comment
        );
        self.write_line(line);
    }

    /// 发射一条 RM 格式指令（op r, d(s)）。
    fn emit_rm(&mut self, op: &str, r: i32, d: i32, s: i32, comment: &str) {
        let line = format!(
            "{:>3}:  {:<5} {},{}({})\t* {}",
            self.emit_loc, op, r, d, s, comment
        );
        self.write_line(line);
    }

    /// 发射一条以绝对地址为目标的 RM 指令。
    ///
    /// 自动将绝对地址换算为相对 PC 的偏移量。
    fn emit_rm_abs(&mut self, op: &str, r: i32, a: i32, comment: &str) {
        let offset = a - (self.emit_loc + 1);
        self.emit_rm(op, r, offset, PC, comment);
    }

    /// 预留 `how_many` 条指令的空白，返回起始位置。
    fn emit_skip(&mut self, how_many: i32) -> i32 {
        let i = self.emit_loc;
        self.emit_loc += how_many;
        if self.high_emit_loc < self.emit_loc {
            self.high_emit_loc = self.emit_loc;
        }
        i
    }

    /// 回退 emit_loc 到指定位置（用于 backpatch）。
    fn emit_backup(&mut self, loc: i32) {
        if loc > self.high_emit_loc {
            panic!("BUG: emit_backup past high_emit_loc");
        }
        self.emit_loc = loc;
    }

    /// 恢复 emit_loc 到已发射的最高位置。
    fn emit_restore(&mut self) {
        self.emit_loc = self.high_emit_loc;
    }

    fn write_line(&mut self, line: String) {
        if self.emit_loc as usize == self.code.len() {
            self.code.push(line);
        } else if (self.emit_loc as usize) < self.code.len() {
            self.code[self.emit_loc as usize] = line;
        } else {
            // 填补中间缺失的槽位
            while self.code.len() < self.emit_loc as usize {
                self.code.push(String::new());
            }
            self.code.push(line);
        }

        self.emit_loc += 1;
        if self.high_emit_loc < self.emit_loc {
            self.high_emit_loc = self.emit_loc;
        }
    }

    /// 将所有指令合并为最终 TM 程序文本。
    fn into_string(self) -> String {
        let mut out = String::new();
        if !self.header.is_empty() {
            out.push_str(&self.header.join("\n"));
            out.push('\n');
        }
        out.push_str(&self.code.join("\n"));
        out.push('\n');
        out
    }
}