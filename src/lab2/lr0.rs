use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::types::*;

pub fn get_closure(item_set: &mut ItemSet, grammar: &Grammar) {
    let mut queue: VecDeque<LR0Item> = item_set.core_items.iter().cloned().collect();

    while let Some(item) = queue.pop_front() {
        let prod = &grammar.productions[item.production_id];

        if let Some(next_sym) = prod.body.get(item.dot_position) {
            if grammar.is_nonterminal(next_sym) {
                if let Some(nt) = grammar.symbols.get(next_sym) {
                    for &pid in &nt.productions {
                        let new_item = LR0Item {
                            production_id: pid,
                            dot_position: 0,
                        };
                        if item_set.items.insert(new_item) {
                            queue.push_back(new_item);
                        }
                    }
                }
            }
        }
    }
}

pub fn exhaust_transitions(
    item_set: &ItemSet,
    grammar: &Grammar,
) -> BTreeMap<String, BTreeSet<LR0Item>> {
    let mut transitions: BTreeMap<String, BTreeSet<LR0Item>> = BTreeMap::new();

    for item in &item_set.items {
        let prod = &grammar.productions[item.production_id];

        if let Some(next_sym) = prod.body.get(item.dot_position) {
            transitions
                .entry(next_sym.clone())
                .or_default()
                .insert(LR0Item {
                    production_id: item.production_id,
                    dot_position: item.dot_position + 1,
                });
        }
    }

    transitions
}

pub fn build_lr0_dfa(grammar: &Grammar) -> LR0DFA {
    let mut dfa = LR0DFA {
        item_sets: Vec::new(),
        edges: Vec::new(),
        start_state: 0,
    };

    let mut start_cores: BTreeSet<LR0Item> = BTreeSet::new();
    start_cores.insert(LR0Item {
        production_id: 0,
        dot_position: 0,
    });
    let mut start_set = ItemSet::new(0, start_cores);
    get_closure(&mut start_set, grammar);
    dfa.item_sets.push(start_set);

    let mut idx = 0;
    while idx < dfa.item_sets.len() {
        let cur = dfa.item_sets[idx].clone();

        let trans = exhaust_transitions(&cur, grammar);

        for (sym, cores) in trans {
            let existing = dfa
                .item_sets
                .iter()
                .position(|s| s.core_items == cores);

            let to_id = match existing {
                Some(id) => id,
                None => {
                    let new_id = dfa.item_sets.len();
                    let mut new_set = ItemSet::new(new_id, cores);
                    get_closure(&mut new_set, grammar);
                    dfa.item_sets.push(new_set);
                    new_id
                }
            };

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
