
#![allow(non_snake_case)]

/// 表示状态机（NFA/DFA）中的一条边（转移路径）
#[derive(Clone, Debug)]
pub struct Edge {
    /// 起始状态的 ID
    pub fromState: i32,
    /// 目标状态（下一个状态）的 ID
    pub nextState: i32,
    /// 转移条件（驱动器）的 ID，通常指向一个字符集或特定的字符序列
    pub driverId: i32,
    /// 驱动器的类型，例如 "CHAR" (字符集驱动) 或 "EPSILON" (空转移)
    pub DriverType: String,
}


