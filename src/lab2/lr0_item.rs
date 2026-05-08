#![allow(non_snake_case)]
#![allow(dead_code)]

use std::hash::{Hash, Hasher};

use crate::lab2::production::Production;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ItemType {
    Core,
    NonCore,
}

impl ItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemType::Core => "CORE",
            ItemType::NonCore => "NONCORE",
        }
    }
}

#[derive(Clone, Debug)]
pub struct LR0Item {
    pub nonTerminalSymbol: String,
    pub production: Production,
    pub dotPosition: i64,
    pub itemType: ItemType,
}

impl LR0Item {
    pub fn new(non_terminal: &str, production: Production, dot_position: i64, item_type: ItemType) -> Self {
        Self {
            nonTerminalSymbol: non_terminal.to_string(),
            production,
            dotPosition: dot_position,
            itemType: item_type,
        }
    }
}

impl PartialEq for LR0Item {
    fn eq(&self, other: &Self) -> bool {
        self.nonTerminalSymbol == other.nonTerminalSymbol
            && self.production == other.production
            && self.dotPosition == other.dotPosition
            && self.itemType == other.itemType
    }
}

impl Eq for LR0Item {}

impl Hash for LR0Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.nonTerminalSymbol.hash(state);
        self.production.productionId.hash(state);
        self.dotPosition.hash(state);
        self.itemType.hash(state);
    }
}
