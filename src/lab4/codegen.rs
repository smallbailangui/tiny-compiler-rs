#![allow(dead_code)]

// ============================================================
// SDT (Syntax-Directed Translation) — 第 2 遍: AST → TM 中间代码
//
// 本文件实现属性文法中 .code 属性的计算。
// 对应 parser.rs 头部所列的每条文法产生式的中间代码语义规则。
//
// ── TM 虚拟机寄存器约定 ─────────────────────────────────
//  AC(0)  = 累加器
//  AC1(1) = 辅助累加器
//  GP(5)  = 全局指针（变量存储区基地址）
//  MP(6)  = 内存顶指针（临时变量栈基地址）
//  PC(7)  = 程序计数器
//
// ── 属性计算（.code 综合属性）───────────────────────────
//   每条文法产生式的 .code 属性由其子节点的 .code 拼接而成。
//   约定以 || 表示代码段的顺序拼接（concatenation）。
//
// 说明：下文对每条文法产生式依次给出
// （a）"类型"——该产生式在文法中的角色；
// （b）"含义"——该产生式表达的 TINY 语言语法结构；
// （c）.code 属性的计算公式——从子节点 .code 拼接为父节点 .code 的规则。
//
// （1）program → stmt-seq
//      类型：程序（起始产生式）
//      含义：整个 TINY 源程序的编译入口。将所有顶层语句的代码连接后，
//            末尾追加 HALT 停机指令。
//      program.code = (∀ s ∈ stmt-seq.stmts) s.code || 'HALT'
//
// （2）stmt-seq → stmt₁ ; … ; stmtₙ
//      类型：语句序列
//      含义：用分号分隔的一条或多条语句的串联。用于表示 if 的 then/else 体、
//            repeat 循环体以及程序的顶层语句列表。
//      stmt-seq.code = stmt₁.code || stmt₂.code || … || stmtₙ.code
//
// （3）read-stmt → read id
//      类型：read 读入语句
//      含义：从标准输入读取一个整数，存入变量 id。对应的 TM 指令为 IN 读入
//            到 AC，再 ST 存入 GP 偏移区。
//      read-stmt.code = 'IN AC,0,0' || 'ST AC, lookup(id.name)(GP)'
//
// （4）write-stmt → write E = expr
//      类型：write 写出语句
//      含义：计算表达式 E 的值并输出到标准输出。先生成 E 的代码（结果在 AC），
//            再发 OUT 指令。
//      write-stmt.code = E.code || 'OUT AC,0,0'
//
// （5）assign-stmt → id := E = expr
//      类型：赋值语句
//      含义：计算右部表达式 E 的值，存入变量 id。生成 E 的代码后发 ST 指令
//            将 AC 写入 GP 偏移区。
//      assign-stmt.code = E.code || 'ST AC, lookup(id.name)(GP)'
//
// （6）if-stmt → if cond = expr then S₁ = stmt-seq [ else S₂ = stmt-seq ] end
//      类型：if-then-else 条件语句
//      含义：根据条件 cond 的真假选择执行 then 分支 S₁ 或 else 分支 S₂。
//            实现方式为：计算 cond → AC；若 AC == 0 跳到 else 或 end；
//            执行 then；若有 else，先跳过 else 再执行 else。
//      if-stmt.code =
//          cond.code
//          || 'JEQ AC, L_else_or_end'   // false → 跳过 then
//          || S₁.code
//          || (S₂ ≠ ε ? 'LDA PC, L_end' : ε)
//          || (S₂ ≠ ε ? L_else: S₂.code : ε)
//          || L_end:
//
// （7）repeat-stmt → repeat S = stmt-seq until cond = expr
//      类型：repeat-until 循环语句
//      含义：先执行循环体 S，再计算条件 cond；若 cond 为假（零）则重复。
//            实现方式为：记录循环起始地址 L_start，生成 S 和 cond 的代码后，
//            用 JEQ AC, L_start 回跳。
//      repeat-stmt.code =
//          L_start: || S.code || cond.code || 'JEQ AC, L_start'
//
// （8）Num(v)
//      类型：整数字面量因子
//      含义：加载立即数 v 到累加器 AC。对应 TM 的 LDC 指令。
//      Num(v).code = 'LDC AC, v, 0'
//
// （9）Id(name)
//      类型：变量引用因子
//      含义：从变量 name 的存储位置加载值到累加器 AC。通过符号表查找偏移量，
//            生成 LD AC, offset(GP) 指令。
//      Id(name).code = 'LD AC, lookup(name)(GP)'
//
// （10）Binary { op, left, right }
//       类型：二元运算表达式（算术/关系）
//       含义：对左操作数 left 和右操作数 right 执行二元运算 op。
//             算术运算（+、−、*、/）直接生成对应 TM 算术指令；
//             关系运算（<、=）通过减法 + 条件跳转 + 0/1 赋值序列实现。
//       Binary.code =
//           left.code
//           || 'ST AC, tmp(MP)'      // 左值压栈
//           || right.code
//           || 'LD AC1, tmp(MP)'     // 弹出左值到 AC1
//           || op ∈ {+,−,*,/} ? op_inst(AC, AC1, AC)
//           || op ∈ {<,=}   ? relop_code(op)
//
//       relop_code(op) =
//           'SUB AC, AC1, AC'        // left − right
//           || '(JLT/JEQ) AC, 2(PC)' // 满足 → 跳到加载 1
//           || 'LDC AC, 0, 0'        // 不满足 → false (0)
//           || 'LDA PC, 1(PC)'       // 跳过加载 1
//           || 'LDC AC, 1, 0'        // 满足 → true (1)
//
// ── 符号表 lookup ──────────────────────────────────────
//   symtab : HashMap<id.name, offset>
//   lookup(id.name) ≡ if id.name ∈ symtab
//                       then symtab[id.name]
//                       else symtab.insert(id.name, next_global++)
//
// ── 回填（Backpatching）机制 ────────────────────────────
//  指令地址在向前生成时未知 → emit_skip 预留槽位 →
//  emit_backup / emit_restore 回填。
//
// ============================================================

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