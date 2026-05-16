use std::collections::{BTreeMap, BTreeSet};

use crate::display::fmt_production;
use crate::types::*;

pub fn check_slr1(dfa: &LR0DFA, grammar: &Grammar) -> (bool, Vec<String>) {
    let mut conflicts = Vec::new();

    // 遍历 DFA 的每一个状态（项集）
    for is in &dfa.item_sets {
        // 第一步：收集该状态下的移入候选符号和规约候选产生式
        let mut shift_terms: BTreeSet<String> = BTreeSet::new();
        let mut reduce_pids: Vec<usize> = Vec::new();

        for item in &is.items {
            let prod = &grammar.productions[item.production_id];
            if let Some(next_sym) = prod.body.get(item.dot_position) {
                // 圆点后有符号
                if grammar.is_terminal(next_sym) {
                    // 该终结符是可能的移入对象
                    shift_terms.insert(next_sym.clone());
                }
            } else {
                // 圆点在末尾 → 可规约（排除增广产生式 0，它只在接受时规约）
                if item.production_id != 0 {
                    reduce_pids.push(item.production_id);
                }
            }
        }

        // 第二步：检测移入-规约冲突
        // 规约项 A→α· 的 FOLLOW(A) 若与移入终结符有交集 → 冲突
        for &rpid in &reduce_pids {
            let rhead = grammar.productions[rpid].head.clone();
            let follow = grammar
                .symbols
                .get(&rhead)
                .map(|s| s.follow_set.clone())
                .unwrap_or_default();
            for t in &shift_terms {
                if follow.contains(t) {
                    // 发现移入-规约冲突：同一个终结符 t 既可以是移入符号，又在 FOLLOW 中
                    conflicts.push(format!(
                        "状态 I{}：移入-规约冲突（规约项 {} 与移入终结符 '{}'）",
                        is.state_id,
                        fmt_production(&grammar.productions[rpid]),
                        t
                    ));
                }
            }
        }

        // 第三步：检测规约-规约冲突
        // 两个规约项的 FOLLOW 集有交集 → 冲突
        for i in 0..reduce_pids.len() {
            for j in (i + 1)..reduce_pids.len() {
                let p1 = reduce_pids[i];
                let p2 = reduce_pids[j];
                let h1 = grammar.productions[p1].head.clone();
                let h2 = grammar.productions[p2].head.clone();
                let f1 = grammar
                    .symbols
                    .get(&h1)
                    .map(|s| s.follow_set.clone())
                    .unwrap_or_default();
                let f2 = grammar
                    .symbols
                    .get(&h2)
                    .map(|s| s.follow_set.clone())
                    .unwrap_or_default();
                // BTreeSet::intersection 求交集 — O(n) 且自动有序
                let inter: BTreeSet<_> = f1.intersection(&f2).cloned().collect();
                if !inter.is_empty() {
                    let inter_str: Vec<String> = inter.into_iter().collect();
                    conflicts.push(format!(
                        "状态 I{}：规约-规约冲突（产生式 {} 与 产生式 {}；FOLLOW 交集 {{{}}}）",
                        is.state_id,
                        p1,
                        p2,
                        inter_str.join(", ")
                    ));
                }
            }
        }
    }

    // 无冲突 → 是 SLR(1) 文法
    (conflicts.is_empty(), conflicts)
}

pub fn build_lr_parse_table(dfa: &LR0DFA, grammar: &Grammar) -> LRParseTable {
    // ACTION 表：键为 (状态编号, 终结符名)，值为动作类型 + id
    let mut action: BTreeMap<(usize, String), ActionCell> = BTreeMap::new();
    // GOTO 表：键为 (状态编号, 非终结符名)，值为下一状态编号
    let mut goto: BTreeMap<(usize, String), GotoCell> = BTreeMap::new();
    // 填表过程中检测到的冲突
    let mut conflicts: Vec<String> = Vec::new();

    // ===== 步骤 1：从 DFA 变迁边填写 ACTION/GOTO =====
    for edge in &dfa.edges {
        if grammar.is_terminal(&edge.driver_symbol) {
            // 终结符 → ACTION 表的 move（移入）动作
            let key = (edge.from_state, edge.driver_symbol.clone());
            let cell = ActionCell {
                action_type: ActionCategory::Shift,
                id: edge.to_state, // 移入后进入的状态编号
            };
            // 检查该位置是否已存在表项（检测冲突）
            if let Some(existing) = action.get(&key) {
                conflicts.push(format!(
                    "ACTION[{}, {}] 移入冲突：原 {:?}{}, 新 s{}",
                    edge.from_state,
                    edge.driver_symbol,
                    existing.action_type,
                    existing.id,
                    edge.to_state
                ));
            }
            action.insert(key, cell);
        } else {
            // 非终结符 → GOTO 表
            goto.insert(
                (edge.from_state, edge.driver_symbol.clone()),
                GotoCell {
                    next_state: edge.to_state,
                },
            );
        }
    }

    // ===== 步骤 2：处理规约项和接受项 =====
    for is in &dfa.item_sets {
        for item in &is.items {
            let prod = &grammar.productions[item.production_id];

            // 圆点已到达产生式末尾 → 可规约或接受
            if item.dot_position >= prod.body.len() {
                if item.production_id == 0 {
                    // 增广产生式 S' → S·：对应接受动作
                    let key = (is.state_id, END_MARKER.to_string());
                    if let Some(existing) = action.get(&key) {
                        conflicts.push(format!(
                            "ACTION[{}, $] 与已有 {:?}{} 冲突（accept）",
                            is.state_id, existing.action_type, existing.id
                        ));
                    }
                    action.insert(
                        key,
                        ActionCell {
                            action_type: ActionCategory::Accept,
                            id: 0,
                        },
                    );
                } else {
                    // 普通规约 A → α·：对每个 b ∈ FOLLOW(A)，填 r
                    let head = prod.head.clone();
                    let follow = grammar
                        .symbols
                        .get(&head)
                        .map(|s| s.follow_set.clone())
                        .unwrap_or_default();
                    for t in &follow {
                        let key = (is.state_id, t.clone());
                        // 检查冲突：同一位置已有表项
                        if let Some(existing) = action.get(&key) {
                            conflicts.push(format!(
                                "ACTION[{}, {}] 与已有 {:?}{} 冲突（reduce r{}）",
                                is.state_id,
                                t,
                                existing.action_type,
                                existing.id,
                                item.production_id
                            ));
                        }
                        action.insert(
                            key,
                            ActionCell {
                                action_type: ActionCategory::Reduce,
                                id: item.production_id, // 规约时 id 存储产生式编号
                            },
                        );
                    }
                }
            }
        }
    }

    LRParseTable {
        action,
        goto,
        conflicts,
    }
}
