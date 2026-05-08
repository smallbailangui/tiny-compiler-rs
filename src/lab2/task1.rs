#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};

use crate::lab2::grammar_symbol::GrammarSymbol;
use crate::lab2::non_terminal_symbol::NonTerminalSymbol;
use crate::lab2::production::Production;
use crate::lab2::supplementary::{
    ts_null, ALL_NON_TERMINAL_SYMBOL_SET, ALL_PRODUCTION_SET, NONE_S,
};
use crate::lab2::terminal_symbol::TerminalSymbol;

fn find_nts_index(all_nts: &[NonTerminalSymbol], name: &str) -> Option<usize> {
    all_nts.iter().position(|nts| nts.name == name)
}

pub fn show_All_nts_production() {
    let all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
    for nts in all_nts.iter() {
        println!("name: {} ", nts.name);
        for p in nts.pProductionTable.iter() {
            print!("   ");
            for s in p.pBodySymbolTable.iter() {
                print!("{} ", s.name());
            }
            println!();
        }
    }
}

pub fn get_Production_FIRST(production: &mut Production) {
    let mut null_stand = true;
    let mut i: usize = 0;
    let ts_null = ts_null();

    while null_stand && i < production.pBodySymbolTable.len() {
        let curr_symbol = production.pBodySymbolTable[i].clone();
        match curr_symbol {
            GrammarSymbol::Terminal(ts) => {
                null_stand = false;
                production.pFirstSet.insert(ts);
            }
            GrammarSymbol::NonTerminal(name) => {
                let all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
                if let Some(idx) = find_nts_index(&all_nts, &name) {
                    let dep_first = all_nts[idx].pFirstSet.clone();
                    drop(all_nts);
                    production.pFirstSet.extend(dep_first.into_iter());
                    production.pFirstSet.remove(&ts_null);
                    let all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
                    if let Some(dep_idx) = find_nts_index(&all_nts, &name) {
                        if !all_nts[dep_idx].pFirstSet.contains(&ts_null) {
                            null_stand = false;
                        }
                    }
                }
            }
            GrammarSymbol::Null => {
                production.pFirstSet.insert(ts_null.clone());
            }
        }
        i += 1;
    }

    if null_stand && i == production.pBodySymbolTable.len() {
        production.pFirstSet.insert(ts_null);
    }
}

pub fn get_All_Production_FIRST() {
    let mut all_production = ALL_PRODUCTION_SET.lock().unwrap();
    for production in all_production.iter_mut() {
        get_Production_FIRST(production);
    }
}

pub fn show_All_Production_FIRST() {
    let all_production = ALL_PRODUCTION_SET.lock().unwrap();
    for production in all_production.iter() {
        print!("PRODUCTION: ");
        for grammar in production.pBodySymbolTable.iter() {
            print!("{} ", grammar.name());
        }
        println!();
        print!("FIRST: ");
        for ts in production.pFirstSet.iter() {
            print!("{} ", ts.name);
        }
        println!();
    }
}

pub fn get_All_NTS_FIRST() {
    let mut all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
    let ts_null = ts_null();
    let mut changed = true;
    while changed {
        changed = false;
        for idx in 0..all_nts.len() {
            let productions = all_nts[idx].pProductionTable.clone();
            for production in productions.iter() {
                let mut can_be_null = true;
                if production.pBodySymbolTable.is_empty() {
                    let before = all_nts[idx].pFirstSet.len();
                    all_nts[idx].pFirstSet.insert(ts_null.clone());
                    if all_nts[idx].pFirstSet.len() > before {
                        changed = true;
                    }
                    continue;
                }

                for symbol in production.pBodySymbolTable.iter() {
                    match symbol {
                        GrammarSymbol::Terminal(ts) => {
                            let before = all_nts[idx].pFirstSet.len();
                            all_nts[idx].pFirstSet.insert(ts.clone());
                            if all_nts[idx].pFirstSet.len() > before {
                                changed = true;
                            }
                            can_be_null = false;
                            break;
                        }
                        GrammarSymbol::Null => {
                            let before = all_nts[idx].pFirstSet.len();
                            all_nts[idx].pFirstSet.insert(ts_null.clone());
                            if all_nts[idx].pFirstSet.len() > before {
                                changed = true;
                            }
                            can_be_null = false;
                            break;
                        }
                        GrammarSymbol::NonTerminal(name) => {
                            if let Some(dep_idx) = find_nts_index(&all_nts, name) {
                                let mut dep_first = all_nts[dep_idx].pFirstSet.clone();
                                dep_first.remove(&ts_null);
                                let before = all_nts[idx].pFirstSet.len();
                                all_nts[idx].pFirstSet.extend(dep_first);
                                if all_nts[idx].pFirstSet.len() > before {
                                    changed = true;
                                }
                                if !all_nts[dep_idx].pFirstSet.contains(&ts_null) {
                                    can_be_null = false;
                                    break;
                                }
                            } else {
                                can_be_null = false;
                                break;
                            }
                        }
                    }
                }

                if can_be_null {
                    let before = all_nts[idx].pFirstSet.len();
                    all_nts[idx].pFirstSet.insert(ts_null.clone());
                    if all_nts[idx].pFirstSet.len() > before {
                        changed = true;
                    }
                }
            }
        }
    }
}

pub fn show_All_NTS_FIRST() {
    let all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
    for nts in all_nts.iter() {
        print!("name: {} FIRST: {{", nts.name);
        for ts in nts.pFirstSet.iter() {
            print!(" {} ", ts.name);
        }
        println!("}}");
    }
}

pub fn FIRST(grammar_symbol: &GrammarSymbol) -> HashSet<TerminalSymbol> {
    match grammar_symbol {
        GrammarSymbol::NonTerminal(name) => {
            let all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
            if let Some(idx) = find_nts_index(&all_nts, name) {
                return all_nts[idx].pFirstSet.clone();
            }
            HashSet::new()
        }
        GrammarSymbol::Terminal(ts) => {
            let mut set = HashSet::new();
            set.insert(ts.clone());
            set
        }
        GrammarSymbol::Null => {
            let mut set = HashSet::new();
            set.insert(ts_null());
            set
        }
    }
}

pub fn get_All_NTS_FOLLOW() {
    let mut all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
    let ts_null = ts_null();

    for nts_idx in 0..all_nts.len() {
        let nts_name = all_nts[nts_idx].name.clone();
        let productions = all_nts[nts_idx].pProductionTable.clone();
        for production in productions.iter() {
            if production.pBodySymbolTable.is_empty() {
                continue;
            }
            let last_idx = production.pBodySymbolTable.len() - 1;
            if let GrammarSymbol::NonTerminal(name) = production.pBodySymbolTable[last_idx].clone() {
                if name != nts_name {
                    if let Some(dep_idx) = find_nts_index(&all_nts, &name) {
                        all_nts[dep_idx].pDependentSetInFollow.insert(nts_name.clone());
                    }
                }
            }

            let mut null_stand = true;
            let mut j = last_idx as i64;
            let mut i = j - 1;
            while i >= 0 {
                let curr_symbol = production.pBodySymbolTable[i as usize].clone();
                match curr_symbol {
                    GrammarSymbol::NonTerminal(name) => {
                        if let Some(dep_idx) = find_nts_index(&all_nts, &name) {
                            let mut k = i + 1;
                            while k <= j {
                                let insert_symbol = production.pBodySymbolTable[k as usize].clone();
                                let mut first_set = FIRST(&insert_symbol);
                                first_set.remove(&ts_null);
                                all_nts[dep_idx].pFollowSet.extend(first_set);
                                k += 1;
                            }

                            if !all_nts[dep_idx].pFirstSet.contains(&ts_null) {
                                j = i;
                            }

                            if null_stand {
                                let next_symbol = production.pBodySymbolTable[(i + 1) as usize].clone();
                                if !FIRST(&next_symbol).contains(&ts_null) {
                                    null_stand = false;
                                }
                            }

                            if null_stand && name != nts_name {
                                all_nts[dep_idx].pDependentSetInFollow.insert(nts_name.clone());
                            }
                        }
                    }
                    GrammarSymbol::Terminal(_) | GrammarSymbol::Null => {
                        j = i;
                        if null_stand {
                            null_stand = false;
                        }
                    }
                }
                i -= 1;
            }
        }
    }

    let mut dependence_map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut curr_edge_num = 0_i64;
    let mut before_edge_num = 0_i64;

    for nts in all_nts.iter() {
        dependence_map.insert(nts.name.clone(), nts.pDependentSetInFollow.clone());
        curr_edge_num += nts.pDependentSetInFollow.len() as i64;
        before_edge_num += nts.pDependentSetInFollow.len() as i64;
    }

    let mut accomplishment: HashSet<String> = HashSet::new();
    for nts in all_nts.iter() {
        if nts.pDependentSetInFollow.is_empty() {
            accomplishment.insert(nts.name.clone());
        }
    }

    while accomplishment.len() != all_nts.len() {
        let mut follow_snapshot: HashMap<String, HashSet<TerminalSymbol>> = HashMap::new();
        for nts in all_nts.iter() {
            follow_snapshot.insert(nts.name.clone(), nts.pFollowSet.clone());
        }
        for nts in all_nts.iter_mut() {
            if let Some(deps) = dependence_map.get(&nts.name) {
                for dep in deps.iter() {
                    if accomplishment.contains(dep) {
                        if let Some(follow_set) = follow_snapshot.get(dep) {
                            nts.pFollowSet.extend(follow_set.clone());
                            curr_edge_num -= 1;
                        }
                    }
                }
            }

            if let Some(entry) = dependence_map.get_mut(&nts.name) {
                for accomplished in accomplishment.iter() {
                    entry.remove(accomplished);
                }
                if entry.is_empty() {
                    accomplishment.insert(nts.name.clone());
                }
            }
        }

        if curr_edge_num == before_edge_num {
            let mut deal_nts_name = NONE_S.to_string();
            for nts in all_nts.iter() {
                if !accomplishment.contains(&nts.name) {
                    deal_nts_name = nts.name.clone();
                }
            }

            if deal_nts_name != NONE_S {
                let mut has_show: HashSet<String> = HashSet::new();
                let mut task_queue: VecDeque<String> = VecDeque::new();
                if let Some(deps) = dependence_map.get(&deal_nts_name) {
                    for dep in deps.iter() {
                        if has_show.insert(dep.clone()) {
                            task_queue.push_back(dep.clone());
                        }
                    }
                }

                while let Some(head) = task_queue.pop_front() {
                    if let Some(next_deps) = dependence_map.get(&head) {
                        for dep in next_deps.iter() {
                            if has_show.insert(dep.clone()) {
                                task_queue.push_back(dep.clone());
                            }
                        }
                    }
                }

                if let Some(deal_idx) = find_nts_index(&all_nts, &deal_nts_name) {
                    for nts_name in has_show.iter() {
                        if let Some(dep_idx) = find_nts_index(&all_nts, nts_name) {
                            let follow_clone = all_nts[dep_idx].pFollowSet.clone();
                            all_nts[deal_idx].pFollowSet.extend(follow_clone);
                        }
                    }

                    accomplishment.insert(deal_nts_name.clone());
                    if let Some(entry) = dependence_map.get(&deal_nts_name) {
                        curr_edge_num -= entry.len() as i64;
                    }
                    before_edge_num = curr_edge_num;
                    if let Some(entry) = dependence_map.get_mut(&deal_nts_name) {
                        entry.clear();
                    }
                }
            }
        } else {
            before_edge_num = curr_edge_num;
        }
    }
}

pub fn show_All_NTS_FOLLOW() {
    let all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
    for nts in all_nts.iter() {
        print!("name: {} FOLLOW: {{", nts.name);
        for follow_symbol in nts.pFollowSet.iter() {
            print!(" {} ", follow_symbol.name);
        }
        println!("}}");
    }
}

pub fn test1() {
    let nts_S = NonTerminalSymbol::new("S", "NONTERMINAL");
    let nts_S_idx = crate::lab2::supplementary::register_non_terminal(nts_S);

    let nts_B = NonTerminalSymbol::new("B", "NONTERMINAL");
    let nts_B_idx = crate::lab2::supplementary::register_non_terminal(nts_B);

    let nts_T = NonTerminalSymbol::new("T", "NONTERMINAL");
    let nts_T_idx = crate::lab2::supplementary::register_non_terminal(nts_T);

    let nts_S1 = NonTerminalSymbol::new("S'", "NONTERMINAL");
    let nts_S1_idx = crate::lab2::supplementary::register_non_terminal(nts_S1);

    let ts_plus = TerminalSymbol::new("+", "TERMINAL", "OPERATOR+");
    crate::lab2::supplementary::register_terminal(ts_plus.clone());

    let ts_left_bracket = TerminalSymbol::new("(", "TERMINAL", "LEFT BRACKET");
    crate::lab2::supplementary::register_terminal(ts_left_bracket.clone());

    let ts_right_bracket = TerminalSymbol::new(")", "TERMINAL", "RIGHT BRACKET");
    crate::lab2::supplementary::register_terminal(ts_right_bracket.clone());

    let ts_multiple = TerminalSymbol::new("*", "TERMINAL", "OPERATOR*");
    crate::lab2::supplementary::register_terminal(ts_multiple.clone());

    let ts_a = TerminalSymbol::new("a", "TERMINAL", "ID");
    crate::lab2::supplementary::register_terminal(ts_a.clone());

    let ts_shut = TerminalSymbol::new("$", "TERMINAL", "$");
    crate::lab2::supplementary::register_terminal(ts_shut.clone());

    {
        let mut all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
        all_nts[nts_S_idx].pFollowSet.insert(ts_shut.clone());

        let mut production_1 = Production::new(1, 2);
        production_1
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("T".to_string()));
        production_1
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("B".to_string()));
        all_nts[nts_S_idx].pProductionTable.push(production_1.clone());
        all_nts[nts_S_idx].numOfProduction = 1;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_1);

        let mut production_2 = Production::new(2, 2);
        production_2
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("S'".to_string()));
        production_2
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("B".to_string()));
        all_nts[nts_B_idx].pProductionTable.push(production_2.clone());
        all_nts[nts_B_idx].numOfProduction = 2;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_2);

        let mut production_3 = Production::new(3, 1);
        production_3
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_null()));
        all_nts[nts_B_idx].pProductionTable.push(production_3.clone());
        all_nts[nts_B_idx].numOfProduction = 2;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_3);

        let mut production_4 = Production::new(4, 2);
        production_4
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_plus.clone()));
        production_4
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("S".to_string()));
        all_nts[nts_S1_idx].pProductionTable.push(production_4.clone());
        all_nts[nts_S1_idx].numOfProduction = 2;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_4);

        let mut production_5 = Production::new(5, 2);
        production_5
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("T".to_string()));
        production_5
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("B".to_string()));
        all_nts[nts_S1_idx].pProductionTable.push(production_5.clone());
        all_nts[nts_S1_idx].numOfProduction = 2;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_5);

        let mut production_6 = Production::new(6, 3);
        production_6
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_left_bracket));
        production_6
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("S".to_string()));
        production_6
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_right_bracket));
        all_nts[nts_T_idx].pProductionTable.push(production_6.clone());
        all_nts[nts_T_idx].numOfProduction = 2;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_6);

        let mut production_7 = Production::new(7, 1);
        production_7
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_a));
        all_nts[nts_T_idx].pProductionTable.push(production_7.clone());
        all_nts[nts_T_idx].numOfProduction = 2;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_7);
    }

    get_All_NTS_FIRST();
    show_All_NTS_FIRST();
    get_All_Production_FIRST();
    show_All_Production_FIRST();
    get_All_NTS_FOLLOW();
    show_All_NTS_FOLLOW();
}
