#![allow(non_snake_case)]
#![allow(dead_code)]

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::lab2::action_cell::ActionCell;
use crate::lab2::cell::Cell;
use crate::lab2::goto_cell::GotoCell;
use crate::lab2::item_set::ItemSet;
use crate::lab2::non_terminal_symbol::NonTerminalSymbol;
use crate::lab2::production::Production;
use crate::lab2::production_info::ProductionInfo;
use crate::lab2::terminal_symbol::TerminalSymbol;
use crate::lab2::transition_edge::TransitionEdge;

pub const NON_S: &str = "NONTERMINAL";
pub const TER_S: &str = "TERMINAL";
pub const NULL_S: &str = "NULL";
pub const NONE_S: &str = "NONE";

pub static GET_NTS: Lazy<Mutex<HashMap<String, usize>>> = Lazy::new(|| Mutex::new(HashMap::new()));
pub static GET_TS: Lazy<Mutex<HashMap<String, TerminalSymbol>>> = Lazy::new(|| Mutex::new(HashMap::new()));
pub static P_PARSE_TABLE_OF_LL: Lazy<Mutex<Vec<Vec<Cell>>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static P_ITEM_SET_TABLE: Lazy<Mutex<Vec<ItemSet>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static P_TRANSITION_EDGE_TABLE: Lazy<Mutex<Vec<TransitionEdge>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static P_ACTION_CELL_TABLE: Lazy<Mutex<HashMap<i64, HashMap<TerminalSymbol, ActionCell>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
pub static P_GOTO_CELL_TABLE: Lazy<Mutex<HashMap<i64, HashMap<String, GotoCell>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
pub static P_PRODUCTION_INFO_TABLE: Lazy<Mutex<Vec<ProductionInfo>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

pub static ALL_NON_TERMINAL_SYMBOL_SET: Lazy<Mutex<Vec<NonTerminalSymbol>>> =
    Lazy::new(|| Mutex::new(Vec::new()));
pub static ALL_PRODUCTION_SET: Lazy<Mutex<Vec<Production>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static RELATIVE_ID: Lazy<Mutex<Vec<i64>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub static TS_NULL: Lazy<TerminalSymbol> = Lazy::new(|| TerminalSymbol::new("null", NULL_S, "null"));
pub static NONE_TERMINAL_SYMBOL: Lazy<TerminalSymbol> =
    Lazy::new(|| TerminalSymbol::new(NONE_S, NONE_S, NONE_S));

pub fn none_terminal_symbol() -> TerminalSymbol {
    NONE_TERMINAL_SYMBOL.clone()
}

pub fn ts_null() -> TerminalSymbol {
    TS_NULL.clone()
}

pub fn register_non_terminal(nts: NonTerminalSymbol) -> usize {
    let mut all_nts = ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap();
    let mut get_nts = GET_NTS.lock().unwrap();
    let index = all_nts.len();
    get_nts.insert(nts.name.clone(), index);
    all_nts.push(nts);
    index
}

pub fn register_terminal(ts: TerminalSymbol) {
    let mut get_ts = GET_TS.lock().unwrap();
    get_ts.insert(ts.name.clone(), ts);
}

pub fn get_nts_index(name: &str) -> Option<usize> {
    let get_nts = GET_NTS.lock().unwrap();
    get_nts.get(name).copied()
}

pub fn get_ts(name: &str) -> Option<TerminalSymbol> {
    let get_ts = GET_TS.lock().unwrap();
    get_ts.get(name).cloned()
}

pub fn clear_lab2_tables() {
    GET_NTS.lock().unwrap().clear();
    GET_TS.lock().unwrap().clear();
    P_PARSE_TABLE_OF_LL.lock().unwrap().clear();
    P_ITEM_SET_TABLE.lock().unwrap().clear();
    P_TRANSITION_EDGE_TABLE.lock().unwrap().clear();
    P_ACTION_CELL_TABLE.lock().unwrap().clear();
    P_GOTO_CELL_TABLE.lock().unwrap().clear();
    P_PRODUCTION_INFO_TABLE.lock().unwrap().clear();
    ALL_NON_TERMINAL_SYMBOL_SET.lock().unwrap().clear();
    ALL_PRODUCTION_SET.lock().unwrap().clear();
    RELATIVE_ID.lock().unwrap().clear();
}
