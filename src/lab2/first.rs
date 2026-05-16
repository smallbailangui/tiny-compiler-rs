use std::collections::BTreeSet;

use crate::types::*;

pub fn first_of_sequence(grammar: &Grammar, seq: &[String]) -> BTreeSet<String> {
    let mut result: BTreeSet<String> = BTreeSet::new();

    // 空序列 → FIRST = { ε }
    if seq.is_empty() {
        result.insert(EPSILON.to_string());
        return result;
    }

    let mut all_can_eps = true;

    // 逐符号扫描，只要前一个能推 ε 才继续看后一个
    for sym in seq {
        let fs: BTreeSet<String> = grammar
            .symbols
            .get(sym)
            .map(|s| s.first_set.clone())
            .unwrap_or_default();

        // 把非 ε 的终结符全部加入结果
        for x in &fs {
            if x != EPSILON {
                result.insert(x.clone());
            }
        }

        // 当前符号不能推导出 ε → 停止向后看
        if !fs.contains(EPSILON) {
            all_can_eps = false;
            break;
        }
    }

    // 序列中每个符号都能推 ε → 结果也包含 ε
    if all_can_eps {
        result.insert(EPSILON.to_string());
    }

    result
}

pub fn compute_first_sets(grammar: &mut Grammar) {
    loop {
        let mut changed = false;

        // 快照避免借用冲突：迭代期间不能同时持有不可变借用和可变借用
        let snapshot: Vec<(usize, String, Vec<String>)> = grammar
            .productions
            .iter()
            .map(|p| (p.production_id, p.head.clone(), p.body.clone()))
            .collect();

        for (pid, head, body) in snapshot {
            let new_fs = first_of_sequence(grammar, &body);

            // 将新的 FIRST 终结符分别写入产生式和非终结符的集合
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

        // 不动点：没有任何集合发生变化时结束
        if !changed {
            break;
        }
    }
}
