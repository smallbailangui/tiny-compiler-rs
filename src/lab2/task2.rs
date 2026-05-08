#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use crate::lab2::action_cell::ActionCell;
use crate::lab2::grammar_symbol::GrammarSymbol;
use crate::lab2::item_set::ItemSet;
use crate::lab2::lr0_item::{ItemType, LR0Item};
use crate::lab2::non_terminal_symbol::NonTerminalSymbol;
use crate::lab2::production::Production;
use crate::lab2::supplementary::{
    get_ts, ALL_NON_TERMINAL_SYMBOL_SET, ALL_PRODUCTION_SET, P_ACTION_CELL_TABLE,
    P_GOTO_CELL_TABLE, P_ITEM_SET_TABLE, P_TRANSITION_EDGE_TABLE,
};
use crate::lab2::terminal_symbol::TerminalSymbol;
use crate::lab2::transition_edge::TransitionEdge;

fn find_nts_index(all_nts: &[NonTerminalSymbol], name: &str) -> Option<usize> {
    all_nts.iter().position(|nts| nts.name == name)
}

pub fn test2() {
    // 创建文法 3.6
    let nts_E1 = NonTerminalSymbol::new("E'", "NONTERMINAL");
    let nts_E1_idx = crate::lab2::supplementary::register_non_terminal(nts_E1);

    let nts_E = NonTerminalSymbol::new("E", "NONTERMINAL");
    let nts_E_idx = crate::lab2::supplementary::register_non_terminal(nts_E);

    let nts_T = NonTerminalSymbol::new("T", "NONTERMINAL");
    let nts_T_idx = crate::lab2::supplementary::register_non_terminal(nts_T);

    let nts_F = NonTerminalSymbol::new("F", "NONTERMINAL");
    let nts_F_idx = crate::lab2::supplementary::register_non_terminal(nts_F);

    let ts_plus = TerminalSymbol::new("+", "TERMINAL", "OPERATOR+");
    crate::lab2::supplementary::register_terminal(ts_plus.clone());

    let ts_left_bracket = TerminalSymbol::new("(", "TERMINAL", "LEFT BRACKET");
    crate::lab2::supplementary::register_terminal(ts_left_bracket.clone());

    let ts_right_bracket = TerminalSymbol::new(")", "TERMINAL", "RIGHT BRACKET");
    crate::lab2::supplementary::register_terminal(ts_right_bracket.clone());

    let ts_multiple = TerminalSymbol::new("*", "TERMINAL", "OPERATOR*");
    crate::lab2::supplementary::register_terminal(ts_multiple.clone());

    let ts_id = TerminalSymbol::new("id", "TERMINAL", "ID");
    crate::lab2::supplementary::register_terminal(ts_id.clone());

    let ts_shut = TerminalSymbol::new("$", "TERMINAL", "$");
    crate::lab2::supplementary::register_terminal(ts_shut.clone());

    {
        let mut all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
        all_nts[nts_E_idx].pFollowSet.insert(ts_shut.clone());

        let mut production_0 = Production::new(0, 1);
        production_0
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("E".to_string()));
        production_0
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_shut.clone()));
        all_nts[nts_E1_idx].pProductionTable.push(production_0.clone());
        all_nts[nts_E1_idx].numOfProduction = 1;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_0);
        crate::lab2::supplementary::RELATIVE_ID.lock().unwrap().push(0);

        let mut production_1 = Production::new(1, 3);
        production_1
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("E".to_string()));
        production_1
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_plus.clone()));
        production_1
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("T".to_string()));
        all_nts[nts_E_idx].pProductionTable.push(production_1.clone());
        all_nts[nts_E_idx].numOfProduction = 2;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_1);
        crate::lab2::supplementary::RELATIVE_ID.lock().unwrap().push(1);

        let mut production_2 = Production::new(2, 1);
        production_2
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("T".to_string()));
        all_nts[nts_E_idx].pProductionTable.push(production_2.clone());
        all_nts[nts_E_idx].numOfProduction = 2;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_2);
        crate::lab2::supplementary::RELATIVE_ID.lock().unwrap().push(1);

        let mut production_3 = Production::new(3, 3);
        production_3
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("T".to_string()));
        production_3
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_multiple.clone()));
        production_3
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("F".to_string()));
        all_nts[nts_T_idx].pProductionTable.push(production_3.clone());
        all_nts[nts_T_idx].numOfProduction = 2;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_3);
        crate::lab2::supplementary::RELATIVE_ID.lock().unwrap().push(2);

        let mut production_4 = Production::new(4, 1);
        production_4
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("F".to_string()));
        all_nts[nts_T_idx].pProductionTable.push(production_4.clone());
        all_nts[nts_T_idx].numOfProduction = 2;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_4);
        crate::lab2::supplementary::RELATIVE_ID.lock().unwrap().push(2);

        let mut production_5 = Production::new(5, 3);
        production_5
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_left_bracket));
        production_5
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("E".to_string()));
        production_5
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_right_bracket));
        all_nts[nts_F_idx].pProductionTable.push(production_5.clone());
        all_nts[nts_F_idx].numOfProduction = 2;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_5);
        crate::lab2::supplementary::RELATIVE_ID.lock().unwrap().push(3);

        let mut production_6 = Production::new(6, 1);
        production_6
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_id));
        all_nts[nts_F_idx].pProductionTable.push(production_6.clone());
        all_nts[nts_F_idx].numOfProduction = 2;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_6);
        crate::lab2::supplementary::RELATIVE_ID.lock().unwrap().push(3);
    }

    crate::lab2::task1::get_All_NTS_FIRST();
    crate::lab2::task1::show_All_NTS_FIRST();
    crate::lab2::task1::get_All_Production_FIRST();
    crate::lab2::task1::show_All_Production_FIRST();
    crate::lab2::task1::get_All_NTS_FOLLOW();
    crate::lab2::task1::show_All_NTS_FOLLOW();
}

pub fn test2_1() {
    let all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
    let all_prod = ALL_PRODUCTION_SET.lock().unwrap();

    let I0_0_item = LR0Item::new(&all_nts[0].name, all_prod[0].clone(), 0, ItemType::Core);
    let mut I0 = ItemSet::new(0);
    I0.pItemTable.push(I0_0_item);

    drop(all_nts);
    drop(all_prod);

    getClosure(&mut I0);
    P_ITEM_SET_TABLE.lock().unwrap().push(I0.clone());
    show_ItemSet(&I0);
    exhaustTransition(I0);
    show_All_itemSet();
}

pub fn show_All_itemSet() {
    let item_sets = P_ITEM_SET_TABLE.lock().unwrap();
    for iS in item_sets.iter() {
        println!("L{} :", iS.stateId);
        show_ItemSet(iS);
        println!();
    }
}

pub fn getClosure(itemSet: &mut ItemSet) {
    let mut has_show_set: HashSet<String> = HashSet::new();
    let mut prepare_queue: Vec<String> = Vec::new();

    for item in itemSet.pItemTable.iter() {
        if (item.dotPosition as usize) < item.production.pBodySymbolTable.len() {
            let curr_symbol = item.production.pBodySymbolTable[item.dotPosition as usize].clone();
            if let GrammarSymbol::NonTerminal(name) = curr_symbol {
                if has_show_set.insert(name.clone()) {
                    prepare_queue.push(name);
                }
            }
        }
    }

    while let Some(curr_name) = prepare_queue.pop() {
        let all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
        if let Some(idx) = find_nts_index(&all_nts, &curr_name) {
            let productions = all_nts[idx].pProductionTable.clone();
            drop(all_nts);
            for production in productions.iter() {
                let item = LR0Item::new(&curr_name, production.clone(), 0, ItemType::NonCore);
                itemSet.pItemTable.push(item);
                if let Some(first_symbol) = production.pBodySymbolTable.first() {
                    if let GrammarSymbol::NonTerminal(next_name) = first_symbol.clone() {
                        if has_show_set.insert(next_name.clone()) {
                            prepare_queue.push(next_name);
                        }
                    }
                }
            }
        }
    }
}

pub fn show_ItemSet(itemSet: &ItemSet) {
    for item in itemSet.pItemTable.iter() {
        if item.itemType == ItemType::Core {
            print!("{} ->", item.nonTerminalSymbol);
            let mut i = 0_i64;
            while (i as usize) < item.production.pBodySymbolTable.len() {
                if i == item.dotPosition {
                    print!(" ·");
                }
                print!(" {}", item.production.pBodySymbolTable[i as usize].name());
                i += 1;
            }
            if i == item.dotPosition {
                print!(" ·");
            }
            println!("     CORE");
        }
    }

    for item in itemSet.pItemTable.iter() {
        if item.itemType == ItemType::NonCore {
            print!("{} ->", item.nonTerminalSymbol);
            let mut i = 0_i64;
            while (i as usize) < item.production.pBodySymbolTable.len() {
                if i == item.dotPosition {
                    print!(" ·");
                }
                print!(" {}", item.production.pBodySymbolTable[i as usize].name());
                i += 1;
            }
            if i == item.dotPosition {
                print!(" ·");
            }
            println!("     NONCORE");
        }
    }
}

pub fn exhaustTransition(itemSet0: ItemSet) {
    let mut transition_queue: Vec<ItemSet> = Vec::new();
    transition_queue.push(itemSet0);

    while let Some(itemSet) = transition_queue.pop() {
        let mut drive_nts: HashSet<String> = HashSet::new();
        let mut drive_ts: HashSet<TerminalSymbol> = HashSet::new();

        for item in itemSet.pItemTable.iter() {
            if (item.dotPosition as usize) < item.production.pBodySymbolTable.len() {
                let curr_symbol = item.production.pBodySymbolTable[item.dotPosition as usize].clone();
                match curr_symbol {
                    GrammarSymbol::NonTerminal(name) => {
                        drive_nts.insert(name);
                    }
                    GrammarSymbol::Terminal(ts) => {
                        drive_ts.insert(ts);
                    }
                    GrammarSymbol::Null => {}
                }
            }
        }

        for nts in drive_nts.iter() {
            let mut Inew = ItemSet::new(P_ITEM_SET_TABLE.lock().unwrap().len() as i64);
            for item in itemSet.pItemTable.iter() {
                if (item.dotPosition as usize) < item.production.pBodySymbolTable.len() {
                    let curr_symbol = item.production.pBodySymbolTable[item.dotPosition as usize].clone();
                    if let GrammarSymbol::NonTerminal(name) = curr_symbol {
                        if &name == nts {
                            let new_item = LR0Item::new(
                                &item.nonTerminalSymbol,
                                item.production.clone(),
                                item.dotPosition + 1,
                                ItemType::Core,
                            );
                            Inew.pItemTable.push(new_item);
                        }
                    }
                }
            }

            if !contains(&mut Inew) {
                getClosure(&mut Inew);
                let len = {
                    let mut table = P_ITEM_SET_TABLE.lock().unwrap();
                    table.push(Inew.clone());
                    table.len()
                };
                if len % 50 == 0 {
                    println!("当前项集数量: {}", len);
                }
                transition_queue.push(Inew.clone());
            }
            P_TRANSITION_EDGE_TABLE
                .lock()
                .unwrap()
                .push(TransitionEdge::new(
                    GrammarSymbol::NonTerminal(nts.clone()),
                    itemSet.clone(),
                    Inew,
                ));
        }

        for ts in drive_ts.iter() {
            let mut Inew = ItemSet::new(P_ITEM_SET_TABLE.lock().unwrap().len() as i64);
            for item in itemSet.pItemTable.iter() {
                if (item.dotPosition as usize) < item.production.pBodySymbolTable.len() {
                    let curr_symbol = item.production.pBodySymbolTable[item.dotPosition as usize].clone();
                    if let GrammarSymbol::Terminal(curr_ts) = curr_symbol {
                        if curr_ts == *ts {
                            let new_item = LR0Item::new(
                                &item.nonTerminalSymbol,
                                item.production.clone(),
                                item.dotPosition + 1,
                                ItemType::Core,
                            );
                            Inew.pItemTable.push(new_item);
                        }
                    }
                }
            }

            if !contains(&mut Inew) {
                getClosure(&mut Inew);
                let len = {
                    let mut table = P_ITEM_SET_TABLE.lock().unwrap();
                    table.push(Inew.clone());
                    table.len()
                };
                if len % 50 == 0 {
                    println!("当前项集数量: {}", len);
                }
                transition_queue.push(Inew.clone());
            }
            P_TRANSITION_EDGE_TABLE
                .lock()
                .unwrap()
                .push(TransitionEdge::new(
                    GrammarSymbol::Terminal(ts.clone()),
                    itemSet.clone(),
                    Inew,
                ));
        }
    }
}

pub fn contains(itemSet: &mut ItemSet) -> bool {
    let item_sets = P_ITEM_SET_TABLE.lock().unwrap();
    for iS in item_sets.iter() {
        if iS == itemSet {
            itemSet.stateId = iS.stateId;
            return true;
        }
    }
    false
}

pub fn test2_4() {
    if judge_SLR1() {
        println!();
        println!("当前文法是SLR1文法");
    }
}

pub fn judge_SLR1() -> bool {
    let item_sets = P_ITEM_SET_TABLE.lock().unwrap();
    let all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();

    for item_set in item_sets.iter() {
        let mut all_set: Vec<HashSet<TerminalSymbol>> = Vec::new();
        let mut drive_set: HashSet<TerminalSymbol> = HashSet::new();
        all_set.push(drive_set.clone());

        for item in item_set.pItemTable.iter() {
            if (item.dotPosition as usize) < item.production.pBodySymbolTable.len() {
                let curr_symbol = item.production.pBodySymbolTable[item.dotPosition as usize].clone();
                if let GrammarSymbol::Terminal(curr_ts) = curr_symbol {
                    drive_set.insert(curr_ts);
                }
            } else {
                if let Some(idx) = find_nts_index(&all_nts, &item.nonTerminalSymbol) {
                    all_set.push(all_nts[idx].pFollowSet.clone());
                }
            }
        }
        all_set[0] = drive_set;

        if judgeSetHasUnion(&all_set) {
            return false;
        }
    }

    true
}

pub fn judgeSetHasUnion(ts_set: &[HashSet<TerminalSymbol>]) -> bool {
    let mut i = 0_usize;
    while i < ts_set.len() {
        let mut j = i + 1;
        while j < ts_set.len() {
            for ts_a in ts_set[i].iter() {
                if ts_set[j].contains(ts_a) {
                    return true;
                }
            }
            j += 1;
        }
        i += 1;
    }
    false
}

pub fn test2_5() {
    create_LR1_Analysis_Table();

    let action_table = P_ACTION_CELL_TABLE.lock().unwrap();
    println!("测试ACTION");
    if let Some(map) = action_table.get(&1) {
        if let Some(ts) = get_ts("$") {
            if let Some(cell) = map.get(&ts) {
                println!("{} TYPE:{}", cell.actionId, cell.actionType);
            }
        }
        if let Some(ts) = get_ts("+") {
            if let Some(cell) = map.get(&ts) {
                println!("{}", cell.actionId);
            }
        }
    }
    println!();
    println!("测试GOTO");
    let goto_table = P_GOTO_CELL_TABLE.lock().unwrap();
    if let Some(map) = goto_table.get(&0) {
        if let Some(cell) = map.get("E") {
            println!("{}", cell.nextStateId);
        }
        if let Some(cell) = map.get("T") {
            println!("{}", cell.nextStateId);
        }
    }
    if let Some(map) = goto_table.get(&4) {
        if let Some(cell) = map.get("T") {
            println!("{}", cell.nextStateId);
        }
    }
    println!();
}

pub fn create_LR1_Analysis_Table() {
    let acc_item_set = getAcc();

    {
        let item_sets = P_ITEM_SET_TABLE.lock().unwrap();
        let mut action_table = P_ACTION_CELL_TABLE.lock().unwrap();
        let mut goto_table = P_GOTO_CELL_TABLE.lock().unwrap();
        for item_set in item_sets.iter() {
            action_table.insert(item_set.stateId, HashMap::new());
            goto_table.insert(item_set.stateId, HashMap::new());
        }
    }

    {
        let edges = P_TRANSITION_EDGE_TABLE.lock().unwrap();
        let mut action_table = P_ACTION_CELL_TABLE.lock().unwrap();
        let mut goto_table = P_GOTO_CELL_TABLE.lock().unwrap();
        for edge in edges.iter() {
            match &edge.driverSymbol {
                GrammarSymbol::Terminal(ts) => {
                    let mut new_ac = ActionCell::new(
                        edge.fromItemSet.stateId,
                        edge.driverSymbol.name(),
                        "s",
                        edge.toItemSet.stateId,
                    );
                    if edge.toItemSet.stateId == acc_item_set.stateId {
                        new_ac.actionType = "acc".to_string();
                    }
                    if let Some(map) = action_table.get_mut(&edge.fromItemSet.stateId) {
                        map.insert(ts.clone(), new_ac);
                    }
                }
                GrammarSymbol::NonTerminal(name) => {
                    let new_gt = crate::lab2::goto_cell::GotoCell::new(
                        edge.fromItemSet.stateId,
                        name,
                        edge.toItemSet.stateId,
                    );
                    if let Some(map) = goto_table.get_mut(&edge.fromItemSet.stateId) {
                        map.insert(name.clone(), new_gt);
                    }
                }
                GrammarSymbol::Null => {}
            }
        }
    }

    let item_sets = P_ITEM_SET_TABLE.lock().unwrap();
    let all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
    let mut action_table = P_ACTION_CELL_TABLE.lock().unwrap();

    for item_set in item_sets.iter() {
        for item in item_set.pItemTable.iter() {
            if item.dotPosition as usize == item.production.pBodySymbolTable.len() {
                if let Some(idx) = find_nts_index(&all_nts, &item.nonTerminalSymbol) {
                    for ts in all_nts[idx].pFollowSet.iter() {
                        let new_ac = ActionCell::new(
                            item_set.stateId,
                            &ts.name,
                            "r",
                            item.production.productionId,
                        );
                        if let Some(map) = action_table.get_mut(&item_set.stateId) {
                            map.insert(ts.clone(), new_ac);
                        }
                    }
                }
            }
        }
    }
}

pub fn getAcc() -> ItemSet {
    let item_sets = P_ITEM_SET_TABLE.lock().unwrap();
    for item_set in item_sets.iter() {
        if item_set.pItemTable.len() == 1 {
            let item = &item_set.pItemTable[0];
            if item.dotPosition as usize == item.production.pBodySymbolTable.len() {
                if let Some(last_symbol) = item.production.pBodySymbolTable.last() {
                    if last_symbol.name() == "$" {
                        return item_set.clone();
                    }
                }
            }
        }
    }
    item_sets.get(0).cloned().unwrap_or_else(|| ItemSet::new(0))
}

pub fn test2_6() {
    let mut test_string_array: Vec<String> = Vec::new();
    test_string_array.push("id".to_string());
    test_string_array.push("+".to_string());
    test_string_array.push("id".to_string());
    test_string_array.push("*".to_string());
    test_string_array.push("id".to_string());
    test_string_array.push("$".to_string());

    crate::lab2::task3::judge_Sentence_LR_Grammar(test_string_array);
}
