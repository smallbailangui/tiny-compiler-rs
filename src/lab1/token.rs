#![allow(non_snake_case)]

use super::category::LexemeCategory;

#[derive(Clone, Debug)]
pub struct Token {
    pub lexeme_category: LexemeCategory,
    pub symbol_type: String,
    pub identify: Option<String>,
    pub value: Option<i64>,
}
