use std::collections::{BTreeMap, BTreeSet};

pub const EPSILON: &str = "ε";
pub const END_MARKER: &str = "$";
pub const DOT: &str = "·";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    Terminal,
    NonTerminal,
    Null,
}

#[derive(Debug, Clone)]
pub struct GrammarSymbol {
    pub name: String,
    pub sym_type: SymbolType,
    pub productions: Vec<usize>,
    pub first_set: BTreeSet<String>,
    pub follow_set: BTreeSet<String>,
    pub dependent_in_follow: BTreeSet<String>,
}

#[derive(Debug, Clone)]
pub struct Production {
    pub production_id: usize,
    pub head: String,
    pub body: Vec<String>,
    pub first_set: BTreeSet<String>,
}

impl Production {
    pub fn is_epsilon(&self) -> bool {
        self.body.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct Grammar {
    pub symbols: BTreeMap<String, GrammarSymbol>,
    pub productions: Vec<Production>,
    pub root_symbol: String,
    pub aug_root: String,
    pub terminals: Vec<String>,
    pub non_terminals: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LR0Item {
    pub production_id: usize,
    pub dot_position: usize,
}

#[derive(Debug, Clone)]
pub struct ItemSet {
    pub state_id: usize,
    pub core_items: BTreeSet<LR0Item>,
    pub items: BTreeSet<LR0Item>,
}

impl ItemSet {
    pub fn new(state_id: usize, cores: BTreeSet<LR0Item>) -> Self {
        ItemSet {
            state_id,
            core_items: cores.clone(),
            items: cores,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransitionEdge {
    pub driver_symbol: String,
    pub from_state: usize,
    pub to_state: usize,
}

#[derive(Debug, Clone)]
pub struct LR0DFA {
    pub item_sets: Vec<ItemSet>,
    pub edges: Vec<TransitionEdge>,
    pub start_state: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionCategory {
    Shift,
    Reduce,
    Accept,
}

#[derive(Debug, Clone)]
pub struct ActionCell {
    pub action_type: ActionCategory,
    pub id: usize,
}

#[derive(Debug, Clone)]
pub struct GotoCell {
    pub next_state: usize,
}

#[derive(Debug, Clone)]
pub struct LRParseTable {
    pub action: BTreeMap<(usize, String), ActionCell>,
    pub goto: BTreeMap<(usize, String), GotoCell>,
    pub conflicts: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LL1ParseTable {
    pub cells: BTreeMap<(String, String), Vec<usize>>,
}
