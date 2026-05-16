use crate::first::first_of_sequence;
use crate::types::*;

pub fn compute_follow_sets(grammar: &mut Grammar) {
    let aug = grammar.aug_root.clone();
    let root = grammar.root_symbol.clone();

    // 规则 1：增广开始符和原始开始符都添加 $
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

        // 快照：避免迭代期间借用冲突
        let snapshot: Vec<(String, Vec<String>)> = grammar
            .productions
            .iter()
            .map(|p| (p.head.clone(), p.body.clone()))
            .collect();

        for (head, body) in snapshot {
            // 缓存头部 A 的 FOLLOW 集（后面规则 3 会用到）
            let head_follow = grammar
                .symbols
                .get(&head)
                .map(|s| s.follow_set.clone())
                .unwrap_or_default();

            // 扫描产生式体中的每个符号 Yᵢ
            for i in 0..body.len() {
                let yi = &body[i];
                // 只关心非终结符
                if !grammar.is_nonterminal(yi) {
                    continue;
                }

                // β = 跟在 Yᵢ 后面的符号序列
                let beta = &body[(i + 1)..];
                let first_beta = first_of_sequence(grammar, beta);

                let yi_owned = yi.clone();

                // 规则 2：FIRST(β) \ {ε} 加入 FOLLOW(Yᵢ)
                if let Some(b_sym) = grammar.symbols.get_mut(&yi_owned) {
                    for x in &first_beta {
                        if x != EPSILON {
                            if b_sym.follow_set.insert(x.clone()) {
                                changed = true;
                            }
                        }
                    }
                }

                // 规则 3：β 为空或能推导出 ε → FOLLOW(A) 传入 FOLLOW(Yᵢ)
                if first_beta.contains(EPSILON) || beta.is_empty() {
                    if let Some(b_sym) = grammar.symbols.get_mut(&yi_owned) {
                        // 记录依赖关系（用于调试和 LL(1) 分析）
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

        // 不动点：没有任何 FOLLOW 集发生变化时结束
        if !changed {
            break;
        }
    }
}
