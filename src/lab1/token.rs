#![allow(non_snake_case)]
#![allow(dead_code)]

use super::category::LexemeCategory;

use super::category::LexemeCategory;

#[derive(Clone, Debug)]
pub struct Token {
    /// 词法类别（强类型）
    pub lexeme_category: LexemeCategory,
    /// 语法符号类型（当前保持原有字符串接口）
    pub symbol_type: String,
    /// 标识符/关键字原文
    pub identify: Option<String>,
    /// 常量值（当前主要用于整数）
    pub value: Option<i64>,
}
