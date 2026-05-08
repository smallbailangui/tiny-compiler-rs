#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::lab2::item_set::ItemSet;
use crate::lab2::transition_edge::TransitionEdge;

#[derive(Clone, Debug)]
pub struct DFA {
    pub startupItemSet: ItemSet,
    pub pEdgeTable: Vec<TransitionEdge>,
}

impl DFA {
    pub fn new(startup_item_set: ItemSet, p_edge_table: Vec<TransitionEdge>) -> Self {
        Self {
            startupItemSet: startup_item_set,
            pEdgeTable: p_edge_table,
        }
    }
}
