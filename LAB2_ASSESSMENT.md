# 实验二完成度核对（上下文无关文法的 DFA 构建）


---

## 一、PDF 实验二中要求的实验任务汇总

实验二 §2 列出以下任务：

### 任务 1：FIRST/FOLLOW 函数求解
1. 产生式的 FIRST 函数求解
2. 非终结符的 FIRST 函数求解
3. 非终结符的 FOLLOW 函数求解

### 任务 2：LR 语法分析
1. `void getClosure(ItemSet* itemSet)` — 一个项集中 LR(0) 核心项的闭包求解
2. `void exhaustTransition(ItemSet* itemSet)` — 穷举一个 LR(0) 项集的变迁
3. 文法的 LR(0) 型 DFA 求解
4. 文法是否为 SLR(1) 文法的判断
5. LR 语法分析表的填写

### 任务 3：验证
1. 以算术运算表达式的文法来验证程序代码的正确性
2. 以 TINY 语言的文法来验证程序代码的正确性，并得出 TINY 语言的语法分析表

### PDF 定义的数据结构
- `GrammarSymbol` / `TerminalSymbol` / `NonTerminalSymbol`
- `Production`（含 `pFirstSet`）
- `LR0Item`（含 `ItemCategory`: CORE / NONCORE）
- `ItemSet`（含 `pItemTable`）
- `TransitionEdge`（含 `GrammarSymbol* driverSymbol`）
- `DFA`（含 `startupItemSet` / `pEdgeTable`）
- `ActionCell` / `GotoCell` / LR 语法分析表
- `ProductionInfo` 产生式概述表

---

## 二、已完成项 ✓

### 2.1 数据结构定义（`src/lab2/types.rs` — 113 行）

| PDF 要求 | Rust 实现 | 状态 |
|:---|:---|:---:|
| `SymbolType` 枚举 (TERMINAL/NONTERMINAL/NULL) | `enum SymbolType { Terminal, NonTerminal, Null }` | ✓ |
| `GrammarSymbol` (name, type) | `struct GrammarSymbol` — 含 name, sym_type, productions, first_set, follow_set, dependent_in_follow | ✓ |
| `NonTerminalSymbol` (含 pProductionTable, pFirstSet, pFollowSet, pDependentSetInFollow) | GrammarSymbol 中已包含所有对应字段 | ✓ |
| `Production` (productionId, bodySize, pBodySymbolTable, pFirstSet) | `struct Production` — 含 production_id, head, body (Vec\<String\>), first_set | ✓ |
| `ItemSet` (stateId, pItemTable) | `struct ItemSet` — 含 state_id, core_items, items | ✓ |
| `TransitionEdge` (driverSymbol, fromItemSet, toItemSet) | `struct TransitionEdge` — 含 driver_symbol, from_state, to_state | ✓ |
| `DFA` (startupItemSet, pEdgeTable) | `struct LR0DFA` — 含 item_sets, edges, start_state | ✓ |
| `ActionCell` (stateId, terminalSymbolName, type, id) | `struct ActionCell` — 含 action_type, id | ✓ |
| `GotoCell` (stateId, nonTerminalSymbolName, nextStateId) | `struct GotoCell` — 含 next_state | ✓ |
| `LRParseTable` (ACTION + GOTO) | `struct LRParseTable` — 含 action (BTreeMap), goto (BTreeMap), conflicts | ✓ |
| LL(1) 分析表数据结构 | `struct LL1ParseTable` — cells (BTreeMap) | ✓ |


### 2.2 FIRST 函数求解（`src/lab2/first.rs` — 70 行）

| PDF 要求 | Rust 实现 | 状态 |
|:---|:---|:---:|
| 产生式的 FIRST 函数求解 | `first_of_sequence()` — 对文法符序列求 FIRST | ✓ |
| 非终结符的 FIRST 函数求解 | `compute_first_sets()` — 不动点迭代，同时更新产生式和非终结符的 FIRST 集 | ✓ |

**实现质量**：
- 正确实现了 FIRST 集的不动点迭代算法
- 正确处理了 ε 传递（若序列中前面的符号都可推导 ε 才继续看后面的符号）
- 同时更新产生式的 `first_set` 和非终结符的 `first_set`（PDF 要求的两个集合）

### 2.3 FOLLOW 函数求解（`src/lab2/follow.rs` — 74 行）

| PDF 要求 | Rust 实现 | 状态 |
|:---|:---|:---:|
| 非终结符的 FOLLOW 函数求解 | `compute_follow_sets()` — 完整实现 | ✓ |

**实现质量**：
- 正确实现了 FOLLOW 集的三条规则：
  1. 对开始符（含增广开始符）加入 `$`
  2. A → αBβ：FIRST(β) \ {ε} 加入 FOLLOW(B)
  3. A → αB 或 A → αBβ 且 β⇒*ε：FOLLOW(A) 加入 FOLLOW(B)
- 使用 `dependent_in_follow` 记录依赖关系（对应 PDF 的 `pDependentSetInFollow`）
- 通过 `loop { if !changed { break } }` 的不动点迭代直至收敛
- 函数签名为 `pub fn compute_follow_sets(grammar: &mut Grammar)`（无返回值，直接修改 Grammar）

### 2.4 LL(1) 语法分析表构造（`src/lab2/ll1.rs` — 56 行）

| PDF 要求 | Rust 实现 | 状态 |
|:---|:---|:---:|
| LL(1) 分析表填写 | `build_ll1_table()` / `check_ll1()` — 实现 | ✓ |

### 2.5 LR(0) 项集闭包求解（`src/lab2/lr0.rs` — 102 行）

| PDF 要求 | Rust 实现 | 状态 |
|:---|:---|:---:|
| `void getClosure(ItemSet* itemSet)` | `pub fn get_closure(item_set: &mut ItemSet, grammar: &Grammar)` — 独立公开函数 | ✓ |

**实现质量**：
- 以独立 `pub fn` 暴露，签名与 PDF 一致（原地修改传入的 ItemSet）
- 正确实现了闭包算法：若圆点后是非终结符 B，则将 B 的所有产生式（圆点在开头）加入项集
- 使用 BTreeSet 保证项目有序且去重，使用 VecDeque 作为工作队列
- 区分核心项（core_items）和非核心项（通过闭包扩展产生）

### 2.6 LR(0) DFA 求解（`src/lab2/lr0.rs`）

| PDF 要求 | Rust 实现 | 状态 |
|:---|:---|:---:|
| `void exhaustTransition(ItemSet* itemSet)` | `pub fn exhaust_transitions(item_set: &ItemSet, grammar: &Grammar) -> BTreeMap<String, BTreeSet<LR0Item>>` — 独立公开函数 | ✓ |
| 文法的 LR(0) 型 DFA 求解 | `build_lr0_dfa()` — 完整实现子集构造法 | ✓ |

**实现质量**：
- `exhaust_transitions` 作为独立 `pub fn` 暴露，返回按驱动符分组的 goto 核心项集合
- `build_lr0_dfa()` 正确实现了 LR(0) DFA 的完整构造流程：
  1. 创建增广文法（S' → S）
  2. 初始项集 I₀ = closure({ [S' → ·S] })
  3. 对每个项集，调用 `exhaust_transitions()` 穷举所有可能的驱动符
  4. 计算 goto(I, X) = closure({ [A → αX·β] | [A → α·Xβ] ∈ I })
  5. 若产生新项集则加入工作队列
  6. 为每个 goto 创建变迁边

### 2.7 SLR(1) 文法判断与语法分析表填写（`src/lab2/slr.rs` — 169 行）

| PDF 要求 | Rust 实现 | 状态 |
|:---|:---|:---:|
| 文法是否为 SLR(1) 文法的判断 | `check_slr1(dfa: &LR0DFA, grammar: &Grammar) -> (bool, Vec<String>)` — 通过检查 DFA 项集检测冲突 | ✓ |
| LR 语法分析表的填写 | `build_lr_parse_table(dfa: &LR0DFA, grammar: &Grammar) -> LRParseTable` — 填写 ACTION 和 GOTO 表 | ✓ |

**实现质量**：
- `check_slr1()` 在 DFA 构建完成后独立检测项集内的移入-规约冲突和规约-规约冲突
- `build_lr_parse_table()` 正确实现了 SLR(1) 分析表的构造规则：
  - `[A → α·aβ]` 且 a 为终结符 → ACTION[i, a] = Shift
  - `[A → α·]` → 对 FOLLOW(A) 中每个终结符 a，ACTION[i, a] = Reduce
  - `[S' → S·]` → ACTION[i, $] = Accept
  - 变迁边中非终结符驱动的 goto → GOTO[i, X] = j
- 正确实现冲突检测（移入-规约冲突、规约-规约冲突）
- 返回冲突列表 `conflicts`，用于 SLR(1) 判定

### 2.8 文法构造（`src/lab2/grammar.rs` — 106 行）

| PDF 要求 | Rust 实现 | 状态 |
|:---|:---|:---:|
| 从非终结符/终结符/产生式列表构建文法 | `Grammar::new()` — 自动创建增广文法、初始化 FIRST/FOLLOW 集 | ✓ |

### 2.9 可视化与报告生成（`src/lab2/display.rs` + `src/lab2/report.rs`）

| 功能 | 实现 | 状态 |
|:---|:---|:---:|
| 终端打印（产生式表、FIRST/FOLLOW、DFA、LR/LL 分析表） | `display.rs` (173 行) | ✓ |
| HTML 报告生成（含 Mermaid DFA 状态图） | `report.rs` (376 行) | ✓ |

### 2.10 验证文法的构造（`src/lab2/tests.rs` — 99 行）

| 文法 | Rust 实现 | 状态 |
|:---|:---|:---:|
| LL(1) 测试文法 1: S → +SS \| \*SS \| a | `make_ll1_test1()` | ✓ |
| LL(1) 测试文法 2: T → S, S → (T)TS \| ε | `make_ll1_test2()` | ✓ |
| 算术表达式文法（基础）: E → E+T \| T, T → T\*F \| F, F → (E) \| id | `make_arith_grammar()` | ✓ |
| 算术表达式文法（扩展 -, /） | `make_arith_extended_grammar()` | ✓ |
| TINY 语言文法 | `make_tiny_grammar()` | ✓ |

**说明**：`tests.rs` 中的函数均为文法工厂函数（返回 `Grammar`），而非 `#[test]` 标注的单元测试。实际测试通过 `runner::run_lab2()` 统一执行，输出 HTML 报告到 `results/lab2_report.html`。因此执行 `cargo test` 不会运行这些验证。

---

