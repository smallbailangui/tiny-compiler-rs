#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::HashSet;

use crate::lab2::grammar_symbol::GrammarSymbol;
use crate::lab2::terminal_symbol::TerminalSymbol;

#[derive(Clone, Debug)]
pub struct Production {
    pub productionId: i64,
    pub bodySize: i64,
    pub pBodySymbolTable: Vec<GrammarSymbol>,
    pub pFirstSet: HashSet<TerminalSymbol>,
}

impl Production {
    pub fn new(production_id: i64, body_size: i64) -> Self {
        Self {
            productionId: production_id,
            bodySize: body_size,
            pBodySymbolTable: Vec::new(),
            pFirstSet: HashSet::new(),
        }
    }
}

impl PartialEq for Production {
    fn eq(&self, other: &Self) -> bool {
        self.productionId == other.productionId
    }
}

impl Eq for Production {}
