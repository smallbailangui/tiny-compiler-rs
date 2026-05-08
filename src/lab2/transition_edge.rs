#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::lab2::grammar_symbol::GrammarSymbol;
use crate::lab2::item_set::ItemSet;

#[derive(Clone, Debug)]
pub struct TransitionEdge {
    pub driverSymbol: GrammarSymbol,
    pub fromItemSet: ItemSet,
    pub toItemSet: ItemSet,
}

impl TransitionEdge {
    pub fn new(driver_symbol: GrammarSymbol, from_item_set: ItemSet, to_item_set: ItemSet) -> Self {
        Self {
            driverSymbol: driver_symbol,
            fromItemSet: from_item_set,
            toItemSet: to_item_set,
        }
    }
}
