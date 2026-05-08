#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::lab2::grammar_symbol::GrammarSymbol;
use crate::lab2::item_set::ItemSet;
use crate::lab2::lr0_item::{ItemType, LR0Item};
use crate::lab2::non_terminal_symbol::NonTerminalSymbol;
use crate::lab2::production::Production;
use crate::lab2::supplementary::{
    clear_lab2_tables, get_ts, register_non_terminal, register_terminal, ALL_NON_TERMINAL_SYMBOL_SET,
    ALL_PRODUCTION_SET, P_ACTION_CELL_TABLE, P_GOTO_CELL_TABLE, P_ITEM_SET_TABLE, RELATIVE_ID,
};
use crate::lab2::terminal_symbol::TerminalSymbol;

pub fn create_Arithmetic_Grammar() {
    let nts_E1 = NonTerminalSymbol::new("E'", "NONTERMINAL");
    let nts_E1_idx = register_non_terminal(nts_E1);

    let nts_E = NonTerminalSymbol::new("E", "NONTERMINAL");
    let nts_E_idx = register_non_terminal(nts_E);

    let nts_T = NonTerminalSymbol::new("T", "NONTERMINAL");
    let nts_T_idx = register_non_terminal(nts_T);

    let nts_F = NonTerminalSymbol::new("F", "NONTERMINAL");
    let nts_F_idx = register_non_terminal(nts_F);

    let ts_plus = TerminalSymbol::new("+", "TERMINAL", "OPERATOR");
    register_terminal(ts_plus.clone());

    let ts_subtract = TerminalSymbol::new("-", "TERMINAL", "OPERATOR");
    register_terminal(ts_subtract.clone());

    let ts_multiply = TerminalSymbol::new("*", "TERMINAL", "OPERATOR");
    register_terminal(ts_multiply.clone());

    let ts_divide = TerminalSymbol::new("/", "TERMINAL", "OPERATOR");
    register_terminal(ts_divide.clone());

    let ts_num = TerminalSymbol::new("num", "TERMINAL", "NUM");
    register_terminal(ts_num.clone());

    let ts_left_bracket = TerminalSymbol::new("(", "TERMINAL", "OPERATOR");
    register_terminal(ts_left_bracket.clone());

    let ts_right_bracket = TerminalSymbol::new(")", "TERMINAL", "OPERATOR");
    register_terminal(ts_right_bracket.clone());

    let ts_shut = TerminalSymbol::new("$", "TERMINAL", "SHUT");
    register_terminal(ts_shut.clone());

    {
        let mut all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
        all_nts[nts_E_idx].pFollowSet.insert(ts_shut.clone());

        let mut production_0 = Production::new(0, 2);
        production_0
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("E".to_string()));
        production_0
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_shut.clone()));
        all_nts[nts_E1_idx].pProductionTable.push(production_0.clone());
        all_nts[nts_E1_idx].numOfProduction = 1;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_0);
        RELATIVE_ID.lock().unwrap().push(0);

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
        all_nts[nts_E_idx].numOfProduction = 4;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_1);
        RELATIVE_ID.lock().unwrap().push(1);

        let mut production_2 = Production::new(2, 3);
        production_2
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("E".to_string()));
        production_2
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_subtract.clone()));
        production_2
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("T".to_string()));
        all_nts[nts_E_idx].pProductionTable.push(production_2.clone());
        all_nts[nts_E_idx].numOfProduction = 4;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_2);
        RELATIVE_ID.lock().unwrap().push(1);

        let mut production_3 = Production::new(3, 1);
        production_3
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("T".to_string()));
        all_nts[nts_E_idx].pProductionTable.push(production_3.clone());
        all_nts[nts_E_idx].numOfProduction = 4;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_3);
        RELATIVE_ID.lock().unwrap().push(1);

        let mut production_4 = Production::new(4, 1);
        production_4
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_num.clone()));
        all_nts[nts_E_idx].pProductionTable.push(production_4.clone());
        all_nts[nts_E_idx].numOfProduction = 4;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_4);
        RELATIVE_ID.lock().unwrap().push(1);

        let mut production_5 = Production::new(5, 3);
        production_5
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("T".to_string()));
        production_5
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_multiply.clone()));
        production_5
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("F".to_string()));
        all_nts[nts_T_idx].pProductionTable.push(production_5.clone());
        all_nts[nts_T_idx].numOfProduction = 3;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_5);
        RELATIVE_ID.lock().unwrap().push(2);

        let mut production_6 = Production::new(6, 3);
        production_6
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("T".to_string()));
        production_6
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_divide.clone()));
        production_6
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("F".to_string()));
        all_nts[nts_T_idx].pProductionTable.push(production_6.clone());
        all_nts[nts_T_idx].numOfProduction = 3;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_6);
        RELATIVE_ID.lock().unwrap().push(2);

        let mut production_7 = Production::new(7, 1);
        production_7
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("F".to_string()));
        all_nts[nts_T_idx].pProductionTable.push(production_7.clone());
        all_nts[nts_T_idx].numOfProduction = 3;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_7);
        RELATIVE_ID.lock().unwrap().push(2);

        let mut production_8 = Production::new(8, 3);
        production_8
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_left_bracket));
        production_8
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("E".to_string()));
        production_8
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_right_bracket));
        all_nts[nts_F_idx].pProductionTable.push(production_8.clone());
        all_nts[nts_F_idx].numOfProduction = 2;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_8);
        RELATIVE_ID.lock().unwrap().push(3);

        let mut production_9 = Production::new(9, 1);
        production_9
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_num));
        all_nts[nts_F_idx].pProductionTable.push(production_9.clone());
        all_nts[nts_F_idx].numOfProduction = 2;
        ALL_PRODUCTION_SET.lock().unwrap().push(production_9);
        RELATIVE_ID.lock().unwrap().push(3);
    }

    println!("展示所有非终结符的FIRST");
    crate::lab2::task1::get_All_NTS_FIRST();
    crate::lab2::task1::show_All_NTS_FIRST();
    println!();
    println!("展示所有产生式的FIRST");
    crate::lab2::task1::get_All_Production_FIRST();
    crate::lab2::task1::show_All_Production_FIRST();
    println!();
    println!("展示所有非终结符的FOLLOW");
    crate::lab2::task1::get_All_NTS_FOLLOW();
    crate::lab2::task1::show_All_NTS_FOLLOW();
    println!();
}

pub fn create_Arithmetic_Grammar_LR0_DFA() {
    let all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
    let all_prod = ALL_PRODUCTION_SET.lock().unwrap();

    let I0_0_item = LR0Item::new(&all_nts[0].name, all_prod[0].clone(), 0, ItemType::Core);
    let mut I0 = ItemSet::new(0);
    I0.pItemTable.push(I0_0_item);

    drop(all_nts);
    drop(all_prod);

    crate::lab2::task2::getClosure(&mut I0);
    P_ITEM_SET_TABLE.lock().unwrap().push(I0.clone());
    crate::lab2::task2::show_ItemSet(&I0);
    crate::lab2::task2::exhaustTransition(I0);
    crate::lab2::task2::show_All_itemSet();
}

pub fn test_Arithmetic_Operation() {
    create_Arithmetic_Grammar();
    create_Arithmetic_Grammar_LR0_DFA();
    if crate::lab2::task2::judge_SLR1() {
        println!("当前文法是SLR1文法");
    }
    crate::lab2::task2::create_LR1_Analysis_Table();

    let mut testGrammarSequence: Vec<String> = Vec::new();
    testGrammarSequence.push("num".to_string());
    testGrammarSequence.push("*".to_string());
    testGrammarSequence.push("(".to_string());
    testGrammarSequence.push("+".to_string());
    testGrammarSequence.push("+".to_string());
    testGrammarSequence.push("num".to_string());
    testGrammarSequence.push(")".to_string());
    testGrammarSequence.push("+".to_string());
    testGrammarSequence.push("num".to_string());
    testGrammarSequence.push("$".to_string());
    judge_Sentence_LR_Grammar(testGrammarSequence);
}

pub fn judge_Sentence_LR_Grammar(ts_sequence: Vec<String>) {
    let mut step = 1_i64;
    let mut pStateStack: Vec<i64> = Vec::new();
    pStateStack.push(0);

    let mut current_lexeme_pos: usize = 0;
    while !pStateStack.is_empty() {
        println!();
        println!("STEP: {}", step);
        step += 1;
        let current_state = *pStateStack.last().unwrap();
        println!(
            "当前状态：{}  当前驱动非终结符：{}",
            current_state, ts_sequence[current_lexeme_pos]
        );

        let action_table = P_ACTION_CELL_TABLE.lock().unwrap();
        if let Some(ts) = get_ts(&ts_sequence[current_lexeme_pos]) {
            if let Some(map) = action_table.get(&current_state) {
                if let Some(indicator) = map.get(&ts) {
                    println!("action： {} {}", indicator.actionType, indicator.actionId);
                    if indicator.actionType == "s" {
                        pStateStack.push(indicator.actionId);
                        current_lexeme_pos += 1;
                        println!("当前状态栈情况");
                        for st in pStateStack.iter() {
                            print!("{} ", st);
                        }
                        println!();
                    } else if indicator.actionType == "r" {
                        let production = {
                            let all_prod = ALL_PRODUCTION_SET.lock().unwrap();
                            all_prod[indicator.actionId as usize].clone()
                        };
                        println!("当前规约编号：{}", indicator.actionId);
                        println!();
                        let mut times = production.pBodySymbolTable.len();
                        while times != 0 {
                            times -= 1;
                            if pStateStack.is_empty() {
                                println!("输入字符串有语法错误");
                                return;
                            }
                            pStateStack.pop();
                        }

                        let current_state = *pStateStack.last().unwrap();
                        let relative_id = {
                            let rel = RELATIVE_ID.lock().unwrap();
                            rel[indicator.actionId as usize] as usize
                        };
                        let nts_name = {
                            let all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
                            all_nts[relative_id].name.clone()
                        };
                        let goto_table = P_GOTO_CELL_TABLE.lock().unwrap();
                        if let Some(map) = goto_table.get(&current_state) {
                            if let Some(cell) = map.get(&nts_name) {
                                pStateStack.push(cell.nextStateId);
                            } else {
                                println!("输入字符串有语法错误");
                                return;
                            }
                        }
                        println!("当前状态栈情况");
                        for st in pStateStack.iter() {
                            print!("{} ", st);
                        }
                        println!();
                    } else if indicator.actionType == "acc" {
                        println!("语法分析完成,语法正确！");
                        return;
                    }
                    continue;
                }
            }
        }
        println!("输入字符串有语法错误");
        return;
    }
}

pub fn create_TINY_Grammar() {
    let ts_if = TerminalSymbol::new("if", "TERMINAL", "KEYWORD");
    let ts_then = TerminalSymbol::new("then", "TERMINAL", "KEYWORD");
    let ts_end = TerminalSymbol::new("end", "TERMINAL", "KEYWORD");
    let ts_else = TerminalSymbol::new("else", "TERMINAL", "KEYWORD");
    let ts_repeat = TerminalSymbol::new("repeat", "TERMINAL", "KEYWORD");
    let ts_until = TerminalSymbol::new("until", "TERMINAL", "KEYWORD");
    let ts_read = TerminalSymbol::new("read", "TERMINAL", "KEYWORD");
    let ts_write = TerminalSymbol::new("write", "TERMINAL", "KEYWORD");

    let ts_semicolon = TerminalSymbol::new(";", "TERMINAL", "OPERATOR");
    let ts_assign = TerminalSymbol::new(":=", "TERMINAL", "OPERATOR");
    let ts_less = TerminalSymbol::new("<", "TERMINAL", "OPERATOR");
    let ts_equal = TerminalSymbol::new("=", "TERMINAL", "OPERATOR");
    let ts_plus = TerminalSymbol::new("+", "TERMINAL", "OPERATOR");
    let ts_subtract = TerminalSymbol::new("-", "TERMINAL", "OPERATOR");
    let ts_multiply = TerminalSymbol::new("*", "TERMINAL", "OPERATOR");
    let ts_divide = TerminalSymbol::new("/", "TERMINAL", "OPERATOR");
    let ts_left_bracket = TerminalSymbol::new("(", "TERMINAL", "OPERATOR");
    let ts_right_bracket = TerminalSymbol::new(")", "TERMINAL", "OPERATOR");

    let ts_identifier = TerminalSymbol::new("identifier", "TERMINAL", "IDENTIFIER");
    let ts_number = TerminalSymbol::new("Number", "TERMINAL", "NUM");

    let ts_shut = TerminalSymbol::new("$", "TERMINAL", "SHUT");

    for ts in [
        ts_if.clone(),
        ts_then.clone(),
        ts_end.clone(),
        ts_else.clone(),
        ts_repeat.clone(),
        ts_until.clone(),
        ts_read.clone(),
        ts_write.clone(),
        ts_semicolon.clone(),
        ts_assign.clone(),
        ts_less.clone(),
        ts_equal.clone(),
        ts_plus.clone(),
        ts_subtract.clone(),
        ts_multiply.clone(),
        ts_divide.clone(),
        ts_left_bracket.clone(),
        ts_right_bracket.clone(),
        ts_identifier.clone(),
        ts_number.clone(),
        ts_shut.clone(),
    ] {
        register_terminal(ts);
    }

    let nts_Program1 = NonTerminalSymbol::new("Program'", "NONTERMINAL");
    let nts_Program1_idx = register_non_terminal(nts_Program1);

    let nts_Program = NonTerminalSymbol::new("Program", "NONTERMINAL");
    let nts_Program_idx = register_non_terminal(nts_Program);

    let nts_stmt_sequence = NonTerminalSymbol::new("stmt-sequence", "NONTERMINAL");
    let nts_stmt_sequence_idx = register_non_terminal(nts_stmt_sequence);

    let nts_statement = NonTerminalSymbol::new("statement", "NONTERMINAL");
    let nts_statement_idx = register_non_terminal(nts_statement);

    let nts_if_stmt = NonTerminalSymbol::new("if-stmt", "NONTERMINAL");
    let nts_if_stmt_idx = register_non_terminal(nts_if_stmt);

    let nts_repeat_stmt = NonTerminalSymbol::new("repeat-stmt", "NONTERMINAL");
    let nts_repeat_stmt_idx = register_non_terminal(nts_repeat_stmt);

    let nts_assign_stmt = NonTerminalSymbol::new("assign-stmt", "NONTERMINAL");
    let nts_assign_stmt_idx = register_non_terminal(nts_assign_stmt);

    let nts_read_stmt = NonTerminalSymbol::new("read-stmt", "NONTERMINAL");
    let nts_read_stmt_idx = register_non_terminal(nts_read_stmt);

    let nts_write_stmt = NonTerminalSymbol::new("write-stmt", "NONTERMINAL");
    let nts_write_stmt_idx = register_non_terminal(nts_write_stmt);

    let nts_exp = NonTerminalSymbol::new("exp", "NONTERMINAL");
    let nts_exp_idx = register_non_terminal(nts_exp);

    let nts_comparison_op = NonTerminalSymbol::new("comparison-op", "NONTERMINAL");
    let nts_comparison_op_idx = register_non_terminal(nts_comparison_op);

    let nts_simple_exp = NonTerminalSymbol::new("simple-exp", "NONTERMINAL");
    let nts_simple_exp_idx = register_non_terminal(nts_simple_exp);

    let nts_addop = NonTerminalSymbol::new("addop", "NONTERMINAL");
    let nts_addop_idx = register_non_terminal(nts_addop);

    let nts_term = NonTerminalSymbol::new("term", "NONTERMINAL");
    let nts_term_idx = register_non_terminal(nts_term);

    let nts_mulop = NonTerminalSymbol::new("mulop", "NONTERMINAL");
    let nts_mulop_idx = register_non_terminal(nts_mulop);

    let nts_factor = NonTerminalSymbol::new("factor", "NONTERMINAL");
    let nts_factor_idx = register_non_terminal(nts_factor);

    {
        let mut all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
        all_nts[nts_Program_idx].pFollowSet.insert(ts_shut.clone());

        let mut production_0 = Production::new(0, 1);
        production_0
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("Program".to_string()));
        production_0
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_shut.clone()));
        all_nts[nts_Program1_idx]
            .pProductionTable
            .push(production_0.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_0);
        RELATIVE_ID.lock().unwrap().push(0);

        let mut production_1 = Production::new(1, 1);
        production_1
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("stmt-sequence".to_string()));
        all_nts[nts_Program_idx]
            .pProductionTable
            .push(production_1.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_1);
        RELATIVE_ID.lock().unwrap().push(1);

        let mut production_2 = Production::new(2, 3);
        production_2
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("stmt-sequence".to_string()));
        production_2
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_semicolon.clone()));
        production_2
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("statement".to_string()));
        all_nts[nts_stmt_sequence_idx]
            .pProductionTable
            .push(production_2.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_2);
        RELATIVE_ID.lock().unwrap().push(2);

        let mut production_3 = Production::new(3, 1);
        production_3
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("statement".to_string()));
        all_nts[nts_stmt_sequence_idx]
            .pProductionTable
            .push(production_3.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_3);
        RELATIVE_ID.lock().unwrap().push(2);

        let mut production_4 = Production::new(4, 1);
        production_4
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("if-stmt".to_string()));
        all_nts[nts_statement_idx]
            .pProductionTable
            .push(production_4.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_4);
        RELATIVE_ID.lock().unwrap().push(3);

        let mut production_5 = Production::new(5, 1);
        production_5
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("repeat-stmt".to_string()));
        all_nts[nts_statement_idx]
            .pProductionTable
            .push(production_5.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_5);
        RELATIVE_ID.lock().unwrap().push(3);

        let mut production_6 = Production::new(6, 1);
        production_6
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("assign-stmt".to_string()));
        all_nts[nts_statement_idx]
            .pProductionTable
            .push(production_6.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_6);
        RELATIVE_ID.lock().unwrap().push(3);

        let mut production_7 = Production::new(7, 1);
        production_7
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("read-stmt".to_string()));
        all_nts[nts_statement_idx]
            .pProductionTable
            .push(production_7.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_7);
        RELATIVE_ID.lock().unwrap().push(3);

        let mut production_8 = Production::new(8, 1);
        production_8
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("write-stmt".to_string()));
        all_nts[nts_statement_idx]
            .pProductionTable
            .push(production_8.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_8);
        RELATIVE_ID.lock().unwrap().push(3);

        let mut production_9 = Production::new(9, 5);
        production_9
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_if.clone()));
        production_9
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("exp".to_string()));
        production_9
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_then.clone()));
        production_9
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("stmt-sequence".to_string()));
        production_9
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_end.clone()));
        all_nts[nts_if_stmt_idx]
            .pProductionTable
            .push(production_9.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_9);
        RELATIVE_ID.lock().unwrap().push(4);

        let mut production_10 = Production::new(10, 7);
        production_10
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_if.clone()));
        production_10
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("exp".to_string()));
        production_10
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_then.clone()));
        production_10
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("stmt-sequence".to_string()));
        production_10
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_else.clone()));
        production_10
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("stmt-sequence".to_string()));
        production_10
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_end.clone()));
        all_nts[nts_if_stmt_idx]
            .pProductionTable
            .push(production_10.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_10);
        RELATIVE_ID.lock().unwrap().push(4);

        let mut production_11 = Production::new(11, 4);
        production_11
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_repeat.clone()));
        production_11
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("stmt-sequence".to_string()));
        production_11
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_until.clone()));
        production_11
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("exp".to_string()));
        all_nts[nts_repeat_stmt_idx]
            .pProductionTable
            .push(production_11.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_11);
        RELATIVE_ID.lock().unwrap().push(5);

        let mut production_12 = Production::new(12, 3);
        production_12
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_identifier.clone()));
        production_12
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_assign.clone()));
        production_12
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("exp".to_string()));
        all_nts[nts_assign_stmt_idx]
            .pProductionTable
            .push(production_12.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_12);
        RELATIVE_ID.lock().unwrap().push(6);

        let mut production_13 = Production::new(13, 2);
        production_13
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_read.clone()));
        production_13
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_identifier.clone()));
        all_nts[nts_read_stmt_idx]
            .pProductionTable
            .push(production_13.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_13);
        RELATIVE_ID.lock().unwrap().push(7);

        let mut production_14 = Production::new(14, 2);
        production_14
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_write.clone()));
        production_14
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("exp".to_string()));
        all_nts[nts_write_stmt_idx]
            .pProductionTable
            .push(production_14.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_14);
        RELATIVE_ID.lock().unwrap().push(8);

        let mut production_15 = Production::new(15, 3);
        production_15
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("simple-exp".to_string()));
        production_15
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("comparison-op".to_string()));
        production_15
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("simple-exp".to_string()));
        all_nts[nts_exp_idx]
            .pProductionTable
            .push(production_15.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_15);
        RELATIVE_ID.lock().unwrap().push(9);

        let mut production_16 = Production::new(16, 1);
        production_16
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("simple-exp".to_string()));
        all_nts[nts_exp_idx]
            .pProductionTable
            .push(production_16.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_16);
        RELATIVE_ID.lock().unwrap().push(9);

        let mut production_17 = Production::new(17, 1);
        production_17
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_less.clone()));
        all_nts[nts_comparison_op_idx]
            .pProductionTable
            .push(production_17.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_17);
        RELATIVE_ID.lock().unwrap().push(10);

        let mut production_18 = Production::new(18, 1);
        production_18
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_equal.clone()));
        all_nts[nts_comparison_op_idx]
            .pProductionTable
            .push(production_18.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_18);
        RELATIVE_ID.lock().unwrap().push(10);

        let mut production_19 = Production::new(19, 3);
        production_19
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("simple-exp".to_string()));
        production_19
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("addop".to_string()));
        production_19
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("term".to_string()));
        all_nts[nts_simple_exp_idx]
            .pProductionTable
            .push(production_19.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_19);
        RELATIVE_ID.lock().unwrap().push(11);

        let mut production_20 = Production::new(20, 1);
        production_20
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("term".to_string()));
        all_nts[nts_simple_exp_idx]
            .pProductionTable
            .push(production_20.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_20);
        RELATIVE_ID.lock().unwrap().push(11);

        let mut production_21 = Production::new(21, 1);
        production_21
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_plus.clone()));
        all_nts[nts_addop_idx]
            .pProductionTable
            .push(production_21.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_21);
        RELATIVE_ID.lock().unwrap().push(12);

        let mut production_22 = Production::new(22, 1);
        production_22
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_subtract.clone()));
        all_nts[nts_addop_idx]
            .pProductionTable
            .push(production_22.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_22);
        RELATIVE_ID.lock().unwrap().push(12);

        let mut production_23 = Production::new(23, 3);
        production_23
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("term".to_string()));
        production_23
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("mulop".to_string()));
        production_23
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("factor".to_string()));
        all_nts[nts_term_idx]
            .pProductionTable
            .push(production_23.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_23);
        RELATIVE_ID.lock().unwrap().push(13);

        let mut production_24 = Production::new(24, 1);
        production_24
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("factor".to_string()));
        all_nts[nts_term_idx]
            .pProductionTable
            .push(production_24.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_24);
        RELATIVE_ID.lock().unwrap().push(13);

        let mut production_25 = Production::new(25, 1);
        production_25
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_multiply.clone()));
        all_nts[nts_mulop_idx]
            .pProductionTable
            .push(production_25.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_25);
        RELATIVE_ID.lock().unwrap().push(14);

        let mut production_26 = Production::new(26, 1);
        production_26
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_divide.clone()));
        all_nts[nts_mulop_idx]
            .pProductionTable
            .push(production_26.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_26);
        RELATIVE_ID.lock().unwrap().push(14);

        let mut production_27 = Production::new(27, 3);
        production_27
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_left_bracket.clone()));
        production_27
            .pBodySymbolTable
            .push(GrammarSymbol::NonTerminal("exp".to_string()));
        production_27
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_right_bracket.clone()));
        all_nts[nts_factor_idx]
            .pProductionTable
            .push(production_27.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_27);
        RELATIVE_ID.lock().unwrap().push(15);

        let mut production_28 = Production::new(28, 1);
        production_28
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_number.clone()));
        all_nts[nts_factor_idx]
            .pProductionTable
            .push(production_28.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_28);
        RELATIVE_ID.lock().unwrap().push(15);

        let mut production_29 = Production::new(29, 1);
        production_29
            .pBodySymbolTable
            .push(GrammarSymbol::Terminal(ts_identifier.clone()));
        all_nts[nts_factor_idx]
            .pProductionTable
            .push(production_29.clone());
        ALL_PRODUCTION_SET.lock().unwrap().push(production_29);
        RELATIVE_ID.lock().unwrap().push(15);
    }

    crate::lab2::task1::get_All_NTS_FIRST();
    crate::lab2::task1::get_All_Production_FIRST();
    crate::lab2::task1::get_All_NTS_FOLLOW();
}

pub fn create_TINY_LR0_DFA() {
    let all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
    let all_prod = ALL_PRODUCTION_SET.lock().unwrap();

    let I0_0_item = LR0Item::new(&all_nts[0].name, all_prod[0].clone(), 0, ItemType::Core);
    let mut I0 = ItemSet::new(0);
    I0.pItemTable.push(I0_0_item);

    drop(all_nts);
    drop(all_prod);

    crate::lab2::task2::getClosure(&mut I0);
    P_ITEM_SET_TABLE.lock().unwrap().push(I0.clone());
    crate::lab2::task2::exhaustTransition(I0);
    crate::lab2::task2::create_LR1_Analysis_Table();
}

pub fn test_TINY1() {
    let mut testGrammarSequence: Vec<String> = Vec::new();
    testGrammarSequence.push("repeat".to_string());
    testGrammarSequence.push("identifier".to_string());
    testGrammarSequence.push(":=".to_string());
    testGrammarSequence.push("identifier".to_string());
    testGrammarSequence.push("*".to_string());
    testGrammarSequence.push("Number".to_string());
    testGrammarSequence.push("until".to_string());
    testGrammarSequence.push("identifier".to_string());
    testGrammarSequence.push("=".to_string());
    testGrammarSequence.push("Number".to_string());
    testGrammarSequence.push("$".to_string());
    judge_Sentence_LR_Grammar(testGrammarSequence);
}

pub fn test_TINY2() {
    let mut testGrammarSequence: Vec<String> = Vec::new();
    testGrammarSequence.push("read".to_string());
    testGrammarSequence.push("identifier".to_string());
    testGrammarSequence.push(";".to_string());
    testGrammarSequence.push("identifier".to_string());
    testGrammarSequence.push(":=".to_string());
    testGrammarSequence.push("identifier".to_string());
    testGrammarSequence.push("-".to_string());
    testGrammarSequence.push("Number".to_string());
    testGrammarSequence.push(";".to_string());
    testGrammarSequence.push("write".to_string());
    testGrammarSequence.push("identifier".to_string());
    testGrammarSequence.push("$".to_string());
    judge_Sentence_LR_Grammar(testGrammarSequence);
}

pub fn test_TINY3() {
    let mut testGrammarSequence: Vec<String> = Vec::new();
    testGrammarSequence.push("read".to_string());
    testGrammarSequence.push("identifier".to_string());
    testGrammarSequence.push(";".to_string());
    testGrammarSequence.push("if".to_string());
    testGrammarSequence.push("Number".to_string());
    testGrammarSequence.push("<".to_string());
    testGrammarSequence.push("identifier".to_string());
    testGrammarSequence.push("then".to_string());
    testGrammarSequence.push("identifier".to_string());
    testGrammarSequence.push(":=".to_string());
    testGrammarSequence.push("Number".to_string());
    testGrammarSequence.push(";".to_string());
    testGrammarSequence.push("repeat".to_string());
    testGrammarSequence.push("identifier".to_string());
    testGrammarSequence.push(":=".to_string());
    testGrammarSequence.push("identifier".to_string());
    testGrammarSequence.push("*".to_string());
    testGrammarSequence.push("identifier".to_string());
    testGrammarSequence.push(";".to_string());
    testGrammarSequence.push("identifier".to_string());
    testGrammarSequence.push(":=".to_string());
    testGrammarSequence.push("identifier".to_string());
    testGrammarSequence.push("-".to_string());
    testGrammarSequence.push("Number".to_string());
    testGrammarSequence.push("until".to_string());
    testGrammarSequence.push("identifier".to_string());
    testGrammarSequence.push("=".to_string());
    testGrammarSequence.push("Number".to_string());
    testGrammarSequence.push(";".to_string());
    testGrammarSequence.push("write".to_string());
    testGrammarSequence.push("identifier".to_string());
    testGrammarSequence.push("end".to_string());
    testGrammarSequence.push("$".to_string());
    judge_Sentence_LR_Grammar(testGrammarSequence);
}

pub fn reset_lab2_state() {
    clear_lab2_tables();
}
