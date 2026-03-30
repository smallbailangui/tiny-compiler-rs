#![allow(non_snake_case)]

use super::category::LexemeCategory;

#[derive(Clone, Debug)]
pub struct State {
    /// 状态编号（在图内唯一）
    pub stateId: i32,
    /// 状态类型：MATCH / UNMATCH
    pub StateType: String,
    /// 接受态词法类别；非接受态通常为 None
    pub LexemeCategory: Option<LexemeCategory>,
}
