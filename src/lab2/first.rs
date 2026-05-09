use std::collections::BTreeSet;

use crate::types::*;

pub fn first_of_sequence(grammar: &Grammar, seq: &[String]) -> BTreeSet<String> {
    let mut result: BTreeSet<String> = BTreeSet::new();

    if seq.is_empty() {
        result.insert(EPSILON.to_string());
        return result;
    }

    let mut all_can_eps = true;

    for sym in seq {
        let fs: BTreeSet<String> = grammar
            .symbols
            .get(sym)
            .map(|s| s.first_set.clone())
            .unwrap_or_default();

        for x in &fs {
            if x != EPSILON {
                result.insert(x.clone());
            }
        }

        if !fs.contains(EPSILON) {
            all_can_eps = false;
            break;
        }
    }

    if all_can_eps {
        result.insert(EPSILON.to_string());
    }

    result
}

pub fn compute_first_sets(grammar: &mut Grammar) {
    loop {
        let mut changed = false;

        let snapshot: Vec<(usize, String, Vec<String>)> = grammar
            .productions
            .iter()
            .map(|p| (p.production_id, p.head.clone(), p.body.clone()))
            .collect();

        for (pid, head, body) in snapshot {
            let new_fs = first_of_sequence(grammar, &body);

            for x in &new_fs {
                if grammar.productions[pid].first_set.insert(x.clone()) {
                    changed = true;
                }
                if let Some(head_sym) = grammar.symbols.get_mut(&head) {
                    if head_sym.first_set.insert(x.clone()) {
                        changed = true;
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }
}
