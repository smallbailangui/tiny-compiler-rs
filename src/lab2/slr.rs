use std::collections::{BTreeMap, BTreeSet};

use crate::display::fmt_production;
use crate::types::*;

pub fn check_slr1(dfa: &LR0DFA, grammar: &Grammar) -> (bool, Vec<String>) {
    let mut conflicts = Vec::new();

    for is in &dfa.item_sets {
        let mut shift_terms: BTreeSet<String> = BTreeSet::new();
        let mut reduce_pids: Vec<usize> = Vec::new();

        for item in &is.items {
            let prod = &grammar.productions[item.production_id];
            if let Some(next_sym) = prod.body.get(item.dot_position) {
                if grammar.is_terminal(next_sym) {
                    shift_terms.insert(next_sym.clone());
                }
            } else {
                if item.production_id != 0 {
                    reduce_pids.push(item.production_id);
                }
            }
        }

        for &rpid in &reduce_pids {
            let rhead = grammar.productions[rpid].head.clone();
            let follow = grammar
                .symbols
                .get(&rhead)
                .map(|s| s.follow_set.clone())
                .unwrap_or_default();
            for t in &shift_terms {
                if follow.contains(t) {
                    conflicts.push(format!(
                        "状态 I{}：移入-规约冲突（规约项 {} 与移入终结符 '{}'）",
                        is.state_id,
                        fmt_production(&grammar.productions[rpid]),
                        t
                    ));
                }
            }
        }

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

    (conflicts.is_empty(), conflicts)
}

pub fn build_lr_parse_table(dfa: &LR0DFA, grammar: &Grammar) -> LRParseTable {
    let mut action: BTreeMap<(usize, String), ActionCell> = BTreeMap::new();
    let mut goto: BTreeMap<(usize, String), GotoCell> = BTreeMap::new();
    let mut conflicts: Vec<String> = Vec::new();

    for edge in &dfa.edges {
        if grammar.is_terminal(&edge.driver_symbol) {
            let key = (edge.from_state, edge.driver_symbol.clone());
            let cell = ActionCell {
                action_type: ActionCategory::Shift,
                id: edge.to_state,
            };
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
            goto.insert(
                (edge.from_state, edge.driver_symbol.clone()),
                GotoCell {
                    next_state: edge.to_state,
                },
            );
        }
    }

    for is in &dfa.item_sets {
        for item in &is.items {
            let prod = &grammar.productions[item.production_id];

            if item.dot_position >= prod.body.len() {
                if item.production_id == 0 {
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
                    let head = prod.head.clone();
                    let follow = grammar
                        .symbols
                        .get(&head)
                        .map(|s| s.follow_set.clone())
                        .unwrap_or_default();
                    for t in &follow {
                        let key = (is.state_id, t.clone());
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
                                id: item.production_id,
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
