#![allow(non_snake_case)]

#[derive(Clone, Debug)]
pub struct Token {
    pub lexeme_category: String,
    pub symbol_type: String,
    pub identify: Option<String>,
    pub value: Option<i64>,
}
