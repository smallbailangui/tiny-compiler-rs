#![allow(non_snake_case)]

use once_cell::sync::Lazy;
use std::sync::Mutex;

use super::category::LexemeCategory;
use super::graph::Graph;

#[derive(Clone, Debug)]
pub struct RegularExpression {
    pub regularId: i32,
    pub name: String,
    pub operatorSymbol: char,
    pub operandId1: i32,
    pub operandId2: i32,
    pub type1: String,
    pub type2: String,
    pub resultType: String,
    /// 该正则最终产出的词法类别（若有）
    pub LexemeCategory: Option<LexemeCategory>,
    pub pNFA: Graph,
}

impl RegularExpression {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        regularId: i32,
        name: impl Into<String>,
        operatorSymbol: char,
        operandId1: i32,
        operandId2: i32,
        type1: impl Into<String>,
        type2: impl Into<String>,
        resultType: impl Into<String>,
        LexemeCategory: Option<LexemeCategory>,
        pNFA: Graph,
    ) -> Self {
        Self {
            regularId,
            name: name.into(),
            operatorSymbol,
            operandId1,
            operandId2,
            type1: type1.into(),
            type2: type2.into(),
            resultType: resultType.into(),
            LexemeCategory,
            pNFA,
        }
    }
}

static P_REGULAR_TABLE: Lazy<Mutex<Vec<RegularExpression>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn add_regular_expression(expr: RegularExpression) {
    P_REGULAR_TABLE.lock().unwrap().push(expr);
}

pub fn regular_table_snapshot() -> Vec<RegularExpression> {
    P_REGULAR_TABLE.lock().unwrap().clone()
}

pub fn clear_regular_table() {
    P_REGULAR_TABLE.lock().unwrap().clear();
}
