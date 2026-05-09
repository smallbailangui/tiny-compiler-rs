use std::collections::{BTreeMap, BTreeSet};

use crate::types::*;

impl Grammar {
    pub fn new(
        non_terminals: &[&str],
        terminals: &[&str],
        productions: &[(&str, &[&str])],
        root: &str,
    ) -> Self {
        let mut symbols: BTreeMap<String, GrammarSymbol> = BTreeMap::new();
        let mut nt_list: Vec<String> = non_terminals.iter().map(|s| s.to_string()).collect();
        let mut t_list: Vec<String> = terminals.iter().map(|s| s.to_string()).collect();

        let aug = format!("{}'", root);
        if !nt_list.contains(&aug) {
            nt_list.insert(0, aug.clone());
        }
        if !t_list.contains(&END_MARKER.to_string()) {
            t_list.push(END_MARKER.to_string());
        }

        for nt in &nt_list {
            symbols.insert(
                nt.clone(),
                GrammarSymbol {
                    name: nt.clone(),
                    sym_type: SymbolType::NonTerminal,
                    productions: Vec::new(),
                    first_set: BTreeSet::new(),
                    follow_set: BTreeSet::new(),
                    dependent_in_follow: BTreeSet::new(),
                },
            );
        }

        for t in &t_list {
            let mut fs = BTreeSet::new();
            fs.insert(t.clone());
            symbols.insert(
                t.clone(),
                GrammarSymbol {
                    name: t.clone(),
                    sym_type: SymbolType::Terminal,
                    productions: Vec::new(),
                    first_set: fs,
                    follow_set: BTreeSet::new(),
                    dependent_in_follow: BTreeSet::new(),
                },
            );
        }

        let mut prods: Vec<Production> = Vec::new();
        prods.push(Production {
            production_id: 0,
            head: aug.clone(),
            body: vec![root.to_string()],
            first_set: BTreeSet::new(),
        });
        symbols.get_mut(&aug).unwrap().productions.push(0);

        for (i, (head, body)) in productions.iter().enumerate() {
            let pid = i + 1;
            let body_vec: Vec<String> = if body.len() == 1 && body[0] == EPSILON {
                Vec::new()
            } else {
                body.iter().map(|s| s.to_string()).collect()
            };
            prods.push(Production {
                production_id: pid,
                head: head.to_string(),
                body: body_vec,
                first_set: BTreeSet::new(),
            });
            symbols
                .get_mut(*head)
                .unwrap_or_else(|| panic!("未声明的非终结符: {}", head))
                .productions
                .push(pid);
        }

        Grammar {
            symbols,
            productions: prods,
            root_symbol: root.to_string(),
            aug_root: aug,
            terminals: t_list,
            non_terminals: nt_list,
        }
    }

    pub fn is_nonterminal(&self, name: &str) -> bool {
        self.symbols
            .get(name)
            .map(|s| s.sym_type == SymbolType::NonTerminal)
            .unwrap_or(false)
    }

    pub fn is_terminal(&self, name: &str) -> bool {
        self.symbols
            .get(name)
            .map(|s| s.sym_type == SymbolType::Terminal)
            .unwrap_or(false)
    }
}
