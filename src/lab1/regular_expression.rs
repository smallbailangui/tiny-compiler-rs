#![allow(non_snake_case)]

#![allow(dead_code)]



use once_cell::sync::Lazy;

use std::sync::Mutex;



use super::category::LexemeCategory;

use super::graph::Graph;



/// 正则表达式实体类，用于在系统中记录从正则表达式到 NFA / DFA 的映射与构造信息

#[derive(Clone, Debug)]

pub struct RegularExpression {

    /// 正则表达式操作（或者节点）ID

    pub regularId: i32,

    /// 表达式或符号名称

    pub name: String,

    /// 操作符号（如 '*'、'|' 等，用于描述 AST 节点操作）

    pub operatorSymbol: char,

    /// 左操作数的 ID（关联内部节点，无则为默认值）

    pub operandId1: i32,

    /// 右操作数的 ID（如无则通常为 -1）

    pub operandId2: i32,

    /// 左操作数的类型

    pub type1: String,

    /// 右操作数的类型

    pub type2: String,

    /// 结果的类型（如 NFA、CHAR 等组合类型名字）

    pub resultType: String,

    /// 该正则表达式对应的终态产出的词法分类（如果是叶结点的词汇规则的话）

    pub LexemeCategory: Option<LexemeCategory>,

    /// 由该正则表达式节点构成的底层状态机 NFA 或 DFA 图形

    pub pNFA: Graph,

}



impl RegularExpression {

    /// 构造一个新的 RegularExpression 实例

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



/// 全局静态变量，用于存放系统中所有解析与构造过的正则表达式记录

static P_REGULAR_TABLE: Lazy<Mutex<Vec<RegularExpression>>> = Lazy::new(|| Mutex::new(Vec::new()));



/// 添加一个正则表达式对象到全局表

pub fn add_regular_expression(expr: RegularExpression) {

    P_REGULAR_TABLE.lock().unwrap().push(expr);

}



/// 获取全局正则表达式表的一个快照（克隆一份完整列表进行安全读取）

pub fn regular_table_snapshot() -> Vec<RegularExpression> {

    P_REGULAR_TABLE.lock().unwrap().clone()

}



/// 清空全局正则表达式表（常用于多次测试前重置状态）

pub fn clear_regular_table() {

    P_REGULAR_TABLE.lock().unwrap().clear();

}

