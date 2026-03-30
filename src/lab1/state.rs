#![allow(non_snake_case)]

use super::category::LexemeCategory;

#[derive(Clone, Debug)]
pub struct State {
    pub stateId: i32,
    pub StateType: String,
    pub LexemeCategory: Option<LexemeCategory>,
}
