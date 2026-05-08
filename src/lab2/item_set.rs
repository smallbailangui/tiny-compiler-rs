#![allow(non_snake_case)]
#![allow(dead_code)]

use std::hash::{Hash, Hasher};

use crate::lab2::lr0_item::{ItemType, LR0Item};

#[derive(Clone, Debug)]
pub struct ItemSet {
    pub stateId: i64,
    pub pItemTable: Vec<LR0Item>,
}

impl ItemSet {
    pub fn new(state_id: i64) -> Self {
        Self {
            stateId: state_id,
            pItemTable: Vec::new(),
        }
    }

    fn core_items(&self) -> Vec<LR0Item> {
        let mut items: Vec<LR0Item> = self
            .pItemTable
            .iter()
            .filter(|item| item.itemType == ItemType::Core)
            .cloned()
            .collect();
        items.sort_by_key(|item| {
            (
                item.nonTerminalSymbol.clone(),
                item.production.productionId,
                item.dotPosition,
            )
        });
        items
    }
}

impl PartialEq for ItemSet {
    fn eq(&self, other: &Self) -> bool {
        let self_core = self.core_items();
        let other_core = other.core_items();
        self_core == other_core
    }
}

impl Eq for ItemSet {}

impl Hash for ItemSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in self.core_items() {
            item.hash(state);
        }
    }
}
