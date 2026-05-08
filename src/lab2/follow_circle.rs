#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::lab2::grammar_symbol::GrammarSymbol;
use crate::lab2::non_terminal_symbol::NonTerminalSymbol;
use crate::lab2::production::Production;
use crate::lab2::supplementary::{register_non_terminal, register_terminal, ts_null, ALL_NON_TERMINAL_SYMBOL_SET, ALL_PRODUCTION_SET};
use crate::lab2::terminal_symbol::TerminalSymbol;

pub fn create_follow_circle_case() {
    let nts_S = NonTerminalSymbol::new("S", "NONTERMINAL");
    let nts_S_idx = register_non_terminal(nts_S);

    let nts_B = NonTerminalSymbol::new("B", "NONTERMINAL");
    let nts_B_idx = register_non_terminal(nts_B);

    let nts_T = NonTerminalSymbol::new("T", "NONTERMINAL");
    let nts_T_idx = register_non_terminal(nts_T);

    let nts_S1 = NonTerminalSymbol::new("S'", "NONTERMINAL");
    let nts_S1_idx = register_non_terminal(nts_S1);

    let ts_plus = TerminalSymbol::new("+", "TERMINAL", "OPERATOR+");
    register_terminal(ts_plus.clone());

    let ts_left_bracket = TerminalSymbol::new("(", "TERMINAL", "LEFT BRACKET");
    register_terminal(ts_left_bracket.clone());

    let ts_right_bracket = TerminalSymbol::new(")", "TERMINAL", "RIGHT BRACKET");
    register_terminal(ts_right_bracket.clone());

    let ts_multiple = TerminalSymbol::new("*", "TERMINAL", "OPERATOR*");
    register_terminal(ts_multiple.clone());

    let ts_a = TerminalSymbol::new("a", "TERMINAL", "ID");
    register_terminal(ts_a.clone());

    let ts_shut = TerminalSymbol::new("$", "TERMINAL", "$");
    register_terminal(ts_shut.clone());

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

    crate::lab2::task1::get_All_NTS_FIRST();
    crate::lab2::task1::show_All_NTS_FIRST();
    crate::lab2::task1::get_All_Production_FIRST();
    crate::lab2::task1::show_All_Production_FIRST();
    crate::lab2::task1::get_All_NTS_FOLLOW();
    crate::lab2::task1::show_All_NTS_FOLLOW();
}
