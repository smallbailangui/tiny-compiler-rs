#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use crate::lab2::grammar_symbol::GrammarSymbol;
use crate::lab2::non_terminal_symbol::NonTerminalSymbol;
use crate::lab2::production::Production;
use crate::lab2::supplementary::{ts_null, ALL_NON_TERMINAL_SYMBOL_SET, ALL_PRODUCTION_SET};
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

/// Computes FIRST sets for all non-terminals using fixed-point iteration.
///
/// Args:
///   None
///
/// Returns:
///   None
pub fn get_All_NTS_FIRST() {
    let mut all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
    let ts_null = ts_null();
    let mut changed = true;

    while changed {
        changed = false;
        for idx in 0..all_nts.len() {
            let productions = all_nts[idx].pProductionTable.clone();
            for production in productions.iter() {
                if production.pBodySymbolTable.is_empty() {
                    let before = all_nts[idx].pFirstSet.len();
                    all_nts[idx].pFirstSet.insert(ts_null.clone());
                    if all_nts[idx].pFirstSet.len() > before {
                        changed = true;
                    }
                    continue;
                }

                let mut can_be_null = true;
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
                                let has_null = dep_first.remove(&ts_null);
                                let before = all_nts[idx].pFirstSet.len();
                                all_nts[idx].pFirstSet.extend(dep_first);
                                if all_nts[idx].pFirstSet.len() > before {
                                    changed = true;
                                }
                                if !has_null {
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

/// Prints FIRST sets for all non-terminals.
///
/// Args:
///   None
///
/// Returns:
///   None
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

/// 计算所有非终结符的 FOLLOW 集合。
///
/// Args:
///   None
///
/// Returns:
///   None
pub fn get_All_NTS_FOLLOW() {
    let mut all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
    let ts_null = ts_null();
    let mut name_map: HashMap<String, usize> = HashMap::new();
    for (idx, nts) in all_nts.iter().enumerate() {
        name_map.insert(nts.name.clone(), idx);
    }

    let first_snapshot: Vec<HashSet<TerminalSymbol>> =
        all_nts.iter().map(|nts| nts.pFirstSet.clone()).collect();

    let mut changed = true;
    while changed {
        changed = false;
        let follow_snapshot: Vec<HashSet<TerminalSymbol>> =
            all_nts.iter().map(|nts| nts.pFollowSet.clone()).collect();

        for head_idx in 0..all_nts.len() {
            let productions = all_nts[head_idx].pProductionTable.clone();
            for production in productions.iter() {
                let body = &production.pBodySymbolTable;
                for i in 0..body.len() {
                    let b_name = match &body[i] {
                        GrammarSymbol::NonTerminal(name) => name.clone(),
                        _ => continue,
                    };
                    let b_idx = match name_map.get(&b_name) {
                        Some(idx) => *idx,
                        None => continue,
                    };

                    let mut suffix_nullable = true;
                    let mut add_set: HashSet<TerminalSymbol> = HashSet::new();
                    let mut k = i + 1;
                    while k < body.len() {
                        match &body[k] {
                            GrammarSymbol::Terminal(ts) => {
                                add_set.insert(ts.clone());
                                suffix_nullable = false;
                                break;
                            }
                            GrammarSymbol::Null => {
                                add_set.insert(ts_null.clone());
                                suffix_nullable = false;
                                break;
                            }
                            GrammarSymbol::NonTerminal(name) => {
                                if let Some(idx) = name_map.get(name) {
                                    let mut first_set = first_snapshot[*idx].clone();
                                    let has_null = first_set.remove(&ts_null);
                                    add_set.extend(first_set);
                                    if !has_null {
                                        suffix_nullable = false;
                                        break;
                                    }
                                } else {
                                    suffix_nullable = false;
                                    break;
                                }
                            }
                        }
                        k += 1;
                    }

                    add_set.remove(&ts_null);
                    let before = all_nts[b_idx].pFollowSet.len();
                    all_nts[b_idx].pFollowSet.extend(add_set);
                    if all_nts[b_idx].pFollowSet.len() > before {
                        changed = true;
                    }

                    if suffix_nullable {
                        let mut head_follow = follow_snapshot[head_idx].clone();
                        head_follow.remove(&ts_null);
                        let before = all_nts[b_idx].pFollowSet.len();
                        all_nts[b_idx].pFollowSet.extend(head_follow);
                        if all_nts[b_idx].pFollowSet.len() > before {
                            changed = true;
                        }
                    }
                }
            }
        }
    }
}

/// Computes FIRST set for a sequence of grammar symbols.
///
/// Args:
///   seq: The symbol sequence to evaluate.
///   all_nts: The non-terminal table.
///   ts_null: The special null terminal symbol.
///
/// Returns:
///   A tuple of (FIRST set without null, can_derive_null).
fn first_of_sequence(
    seq: &[GrammarSymbol],
    all_nts: &[NonTerminalSymbol],
    ts_null: &TerminalSymbol,
) -> (HashSet<TerminalSymbol>, bool) {
    let mut result: HashSet<TerminalSymbol> = HashSet::new();
    if seq.is_empty() {
        return (result, true);
    }

    for symbol in seq.iter() {
        match symbol {
            GrammarSymbol::Terminal(ts) => {
                result.insert(ts.clone());
                return (result, false);
            }
            GrammarSymbol::Null => {
                return (result, true);
            }
            GrammarSymbol::NonTerminal(name) => {
                if let Some(dep_idx) = find_nts_index(all_nts, name) {
                    let mut dep_first = all_nts[dep_idx].pFirstSet.clone();
                    let has_null = dep_first.remove(ts_null);
                    result.extend(dep_first);
                    if !has_null {
                        return (result, false);
                    }
                } else {
                    return (result, false);
                }
            }
        }
    }

    (result, true)
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
