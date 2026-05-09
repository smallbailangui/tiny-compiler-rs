use std::collections::BTreeMap;

use crate::first::first_of_sequence;
use crate::types::*;

pub fn build_ll1_parse_table(grammar: &Grammar) -> LL1ParseTable {
    let mut cells: BTreeMap<(String, String), Vec<usize>> = BTreeMap::new();

    for prod in &grammar.productions {
        if prod.production_id == 0 {
            continue;
        }

        let first_alpha = first_of_sequence(grammar, &prod.body);

        for t in &first_alpha {
            if t == EPSILON {
                let follow = grammar
                    .symbols
                    .get(&prod.head)
                    .map(|s| s.follow_set.clone())
                    .unwrap_or_default();
                for f in &follow {
                    let entry = cells
                        .entry((prod.head.clone(), f.clone()))
                        .or_default();
                    if !entry.contains(&prod.production_id) {
                        entry.push(prod.production_id);
                    }
                }
            } else {
                let entry = cells
                    .entry((prod.head.clone(), t.clone()))
                    .or_default();
                if !entry.contains(&prod.production_id) {
                    entry.push(prod.production_id);
                }
            }
        }
    }

    LL1ParseTable { cells }
}

pub fn check_ll1(table: &LL1ParseTable) -> (bool, Vec<String>) {
    let mut conflicts = Vec::new();
    for ((nt, t), prods) in &table.cells {
        if prods.len() > 1 {
            conflicts.push(format!(
                "M[{}, {}] 中存在多条产生式 {:?}",
                nt, t, prods
            ));
        }
    }
    (conflicts.is_empty(), conflicts)
}
