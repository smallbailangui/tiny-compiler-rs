use crate::first::first_of_sequence;
use crate::types::*;

pub fn compute_follow_sets(grammar: &mut Grammar) {
    let aug = grammar.aug_root.clone();
    let root = grammar.root_symbol.clone();
    grammar
        .symbols
        .get_mut(&aug)
        .unwrap()
        .follow_set
        .insert(END_MARKER.to_string());
    grammar
        .symbols
        .get_mut(&root)
        .unwrap()
        .follow_set
        .insert(END_MARKER.to_string());

    loop {
        let mut changed = false;

        let snapshot: Vec<(String, Vec<String>)> = grammar
            .productions
            .iter()
            .map(|p| (p.head.clone(), p.body.clone()))
            .collect();

        for (head, body) in snapshot {
            let head_follow = grammar
                .symbols
                .get(&head)
                .map(|s| s.follow_set.clone())
                .unwrap_or_default();

            for i in 0..body.len() {
                let yi = &body[i];
                if !grammar.is_nonterminal(yi) {
                    continue;
                }

                let beta = &body[(i + 1)..];
                let first_beta = first_of_sequence(grammar, beta);

                let yi_owned = yi.clone();

                if let Some(b_sym) = grammar.symbols.get_mut(&yi_owned) {
                    for x in &first_beta {
                        if x != EPSILON {
                            if b_sym.follow_set.insert(x.clone()) {
                                changed = true;
                            }
                        }
                    }
                }

                if first_beta.contains(EPSILON) || beta.is_empty() {
                    if let Some(b_sym) = grammar.symbols.get_mut(&yi_owned) {
                        b_sym.dependent_in_follow.insert(head.clone());
                        for x in &head_follow {
                            if b_sym.follow_set.insert(x.clone()) {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }
}
