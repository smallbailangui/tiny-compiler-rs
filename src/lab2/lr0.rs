use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::types::*;

pub fn get_closure(item_set: &mut ItemSet, grammar: &Grammar) {
    // 工作队列：从核心项开始，广度优先扩展
    let mut queue: VecDeque<LR0Item> = item_set.core_items.iter().cloned().collect();

    while let Some(item) = queue.pop_front() {
        let prod = &grammar.productions[item.production_id];

        // 圆点后有符号 → 检查是否需要闭包扩展
        if let Some(next_sym) = prod.body.get(item.dot_position) {
            // 只对非终结符进行闭包扩展（终结符无产生式）
            if grammar.is_nonterminal(next_sym) {
                if let Some(nt) = grammar.symbols.get(next_sym) {
                    // 将该非终结符的所有产生式以圆点在头部的形式加入
                    for &pid in &nt.productions {
                        let new_item = LR0Item {
                            production_id: pid,
                            dot_position: 0, // 圆点在最左端
                        };
                        // insert 返回 true 表示新项（之前不存在）
                        if item_set.items.insert(new_item) {
                            queue.push_back(new_item);
                        }
                    }
                }
            }
        }
        // 圆点已在末尾 → 不产生新的闭包项
    }
}

pub fn exhaust_transitions(
    item_set: &ItemSet,
    grammar: &Grammar,
) -> BTreeMap<String, BTreeSet<LR0Item>> {
    let mut transitions: BTreeMap<String, BTreeSet<LR0Item>> = BTreeMap::new();

    // 遍历项集中所有项目（含闭包生成的非核心项）
    for item in &item_set.items {
        let prod = &grammar.productions[item.production_id];

        // 圆点后有符号 → 可产生变迁
        if let Some(next_sym) = prod.body.get(item.dot_position) {
            // 按驱动符分组，产生圆点右移一位的新核心项目
            transitions
                .entry(next_sym.clone())
                .or_default()
                .insert(LR0Item {
                    production_id: item.production_id,
                    dot_position: item.dot_position + 1, // 圆点右移
                });
        }
        // 圆点在末尾 → 无变迁（规约项，不移入）
    }

    transitions
}

pub fn build_lr0_dfa(grammar: &Grammar) -> LR0DFA {
    let mut dfa = LR0DFA {
        item_sets: Vec::new(),
        edges: Vec::new(),
        start_state: 0,
    };

    // 步骤 2：初始项集 I₀ = closure({ [S' → ·S] })
    let mut start_cores: BTreeSet<LR0Item> = BTreeSet::new();
    start_cores.insert(LR0Item {
        production_id: 0, // 增广产生式 S' → S
        dot_position: 0,
    });
    let mut start_set = ItemSet::new(0, start_cores);
    get_closure(&mut start_set, grammar);
    dfa.item_sets.push(start_set);

    // 步骤 3-4：BFS 遍历所有项集
    let mut idx = 0;
    while idx < dfa.item_sets.len() {
        let cur = dfa.item_sets[idx].clone();

        // 3a：穷举当前项集的所有变迁（按驱动符分组核心项）
        let trans = exhaust_transitions(&cur, grammar);

        // 3b-3e：对每种驱动符计算 GOTO 并创建/复用状态
        for (sym, cores) in trans {
            // 3c：通过比较核心项判断是否已存在相同项集
            let existing = dfa
                .item_sets
                .iter()
                .position(|s| s.core_items == cores);

            let to_id = match existing {
                Some(id) => id, // 复用已有状态
                None => {
                    // 3d：创建新状态
                    let new_id = dfa.item_sets.len();
                    let mut new_set = ItemSet::new(new_id, cores);
                    get_closure(&mut new_set, grammar);
                    dfa.item_sets.push(new_set);
                    new_id
                }
            };

            // 3e：记录变迁边
            dfa.edges.push(TransitionEdge {
                driver_symbol: sym,
                from_state: idx,
                to_state: to_id,
            });
        }

        idx += 1;
    }

    dfa
}
