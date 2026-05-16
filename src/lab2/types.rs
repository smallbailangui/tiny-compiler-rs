use std::collections::{BTreeMap, BTreeSet};

// ---------- 常量定义 ----------

pub const EPSILON: &str = "ε";
/// $ 结束标记 — 输入串的结束符，用于 LR 分析中表示接受
pub const END_MARKER: &str = "$";
/// · 圆点 — 在 LR(0) 项目中标记当前分析位置
pub const DOT: &str = "·";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    Terminal,    // 终结符 — 不能再展开的文法符号，如 id, +, (
    NonTerminal, // 非终结符 — 可以继续推导的文法符号，如 E, T, F
    Null,        // 空符号 ε — 表示空串
}

#[derive(Debug, Clone)]
pub struct GrammarSymbol {
    pub name: String,                             // 符号名称，如 "E", "id", "+"
    pub sym_type: SymbolType,                     // 符号类别：终结符/非终结符/空
    pub productions: Vec<usize>,                  // 该非终结符对应的产生式编号列表
    pub first_set: BTreeSet<String>,              // FIRST 集（终结符集合）
    pub follow_set: BTreeSet<String>,             // FOLLOW 集（终结符集合）
    pub dependent_in_follow: BTreeSet<String>,    // FOLLOW 求解中所依赖的非终结符
}

#[derive(Debug, Clone)]
pub struct Production {
    pub production_id: usize,          // 产生式序号，从 0 开始，0 号为增广产生式
    pub head: String,                  // 产生式头（左部），非终结符名称
    pub body: Vec<String>,             // 产生式体（右部），文法符序列。空 Vec 表示 ε 产生式
    pub first_set: BTreeSet<String>,   // 该产生式的 FIRST 集
}

impl Production {
    /// 判断该产生式是否为 ε 产生式（产生式体为空）
    pub fn is_epsilon(&self) -> bool {
        self.body.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct Grammar {
    pub symbols: BTreeMap<String, GrammarSymbol>,  // 所有文法符号（键为符号名）
    pub productions: Vec<Production>,               // 产生式表（索引即产生式编号）
    pub root_symbol: String,                        // 原始开始符号
    pub aug_root: String,                           // 增广开始符号（如 E'）
    pub terminals: Vec<String>,                     // 终结符列表
    pub non_terminals: Vec<String>,                 // 非终结符列表
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LR0Item {
    pub production_id: usize,  // 产生式编号，指向 Grammar.productions 的索引
    pub dot_position: usize,   // 圆点位置。0 表示在第一个符号之前，body.len() 表示在末尾
}


#[derive(Debug, Clone)]
pub struct ItemSet {
    pub state_id: usize,                 // 状态序号，DFA 中唯一标识
    pub core_items: BTreeSet<LR0Item>,   // 核心项集合 — 用于判等（子集构造法的关键）
    pub items: BTreeSet<LR0Item>,        // 完整项集合 — 核心项 + 闭包生成的非核心项
}

impl ItemSet {
    /// 创建新项集，核心项初始化为 cores，items 也先从 cores 开始
    pub fn new(state_id: usize, cores: BTreeSet<LR0Item>) -> Self {
        ItemSet {
            state_id,
            core_items: cores.clone(),
            items: cores,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransitionEdge {
    pub driver_symbol: String,  // 驱动符名称（终结符驱动的是 ACTION 边，非终结符驱动的是 GOTO 边）
    pub from_state: usize,      // 出发项集的状态编号
    pub to_state: usize,        // 到达项集的状态编号
}

#[derive(Debug, Clone)]
pub struct LR0DFA {
    pub item_sets: Vec<ItemSet>,            // 所有 LR(0) 项集（按状态编号索引）
    pub edges: Vec<TransitionEdge>,         // 所有变迁边
    pub start_state: usize,                 // 开始状态编号（通常为 0）
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionCategory {
    Shift,   // 移入 — 对应指导书中的 's'，将输入符号和下一状态入栈
    Reduce,  // 规约 — 对应指导书中的 'r'，按产生式规约
    Accept,  // 接受 — 对应指导书中的 'a'，分析成功
}

#[derive(Debug, Clone)]
pub struct ActionCell {
    pub action_type: ActionCategory,  // 动作类别：Shift / Reduce / Accept
    pub id: usize,                    // 动作参数：移入时为目标状态号，规约时为产生式号
}

#[derive(Debug, Clone)]
pub struct GotoCell {
    pub next_state: usize,  // 下一状态编号
}

#[derive(Debug, Clone)]
pub struct LRParseTable {
    pub action: BTreeMap<(usize, String), ActionCell>,  // ACTION 表，键为 (状态号, 终结符)
    pub goto: BTreeMap<(usize, String), GotoCell>,      // GOTO 表，键为 (状态号, 非终结符)
    pub conflicts: Vec<String>,                          // 分析表填写中检测到的冲突信息
}

#[derive(Debug, Clone)]
pub struct LL1ParseTable {
    pub cells: BTreeMap<(String, String), Vec<usize>>,  // LL(1) 分析表，键为 (非终结符, 终结符)
}