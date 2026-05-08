#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use crate::lab2::production::Production;
use crate::lab2::terminal_symbol::TerminalSymbol;

#[derive(Clone, Debug)]
pub struct NonTerminalSymbol {
    pub name: String,
    pub symbolType: String,
    pub pProductionTable: Vec<Production>,
    pub numOfProduction: i64,
    pub pFirstSet: HashSet<TerminalSymbol>,
    pub pFollowSet: HashSet<TerminalSymbol>,
    pub pDependentSetInFollow: HashSet<String>,
}

impl NonTerminalSymbol {
    pub fn new(name: &str, symbol_type: &str) -> Self {
        Self {
            name: name.to_string(),
            symbolType: symbol_type.to_string(),
            pProductionTable: Vec::new(),
            numOfProduction: 0,
            pFirstSet: HashSet::new(),
            pFollowSet: HashSet::new(),
            pDependentSetInFollow: HashSet::new(),
        }
    }
}

impl PartialEq for NonTerminalSymbol {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for NonTerminalSymbol {}

impl Hash for NonTerminalSymbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
