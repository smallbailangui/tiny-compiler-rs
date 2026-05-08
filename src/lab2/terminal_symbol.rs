#![allow(non_snake_case)]
#![allow(dead_code)]

use std::hash::{Hash, Hasher};

use crate::lab1::category::LexemeCategory;

#[derive(Clone, Debug)]
pub struct TerminalSymbol {
    pub name: String,
    pub symbolType: String,
    pub lemexeCategory: LexemeCategory,
}

impl TerminalSymbol {
    pub fn new(name: &str, symbol_type: &str, lexeme_tag: &str) -> Self {
        let category = map_lexeme_category(name, lexeme_tag);
        Self {
            name: name.to_string(),
            symbolType: symbol_type.to_string(),
            lemexeCategory: category,
        }
    }
}

impl PartialEq for TerminalSymbol {
    fn eq(&self, other: &Self) -> bool {
        self.symbolType == other.symbolType && self.name == other.name
    }
}

impl Eq for TerminalSymbol {}

impl Hash for TerminalSymbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbolType.hash(state);
        self.name.hash(state);
    }
}

fn map_lexeme_category(name: &str, lexeme_tag: &str) -> LexemeCategory {
    match lexeme_tag {
        "KEYWORD" => LexemeCategory::KEYWORD,
        "ASSIGN" | "ASSIGN_OPERATOR" => LexemeCategory::ASSIGN_OPERATOR,
        "COMPARE" | "COMPARE_OPERATOR" => LexemeCategory::COMPARE_OPERATOR,
        "OPERATOR+" | "OPERATOR-" | "OPERATOR*" | "OPERATOR/" => {
            LexemeCategory::NUMERIC_OPERATOR
        }
        "OPERATOR" => match name {
            "+" | "-" | "*" | "/" => LexemeCategory::NUMERIC_OPERATOR,
            "<" | "=" => LexemeCategory::COMPARE_OPERATOR,
            ":=" => LexemeCategory::ASSIGN_OPERATOR,
            "(" | ")" | ";" => LexemeCategory::LOGIC_OPERATOR,
            _ => LexemeCategory::LOGIC_OPERATOR,
        },
        "LEFT BRACKET" | "RIGHT BRACKET" | "SHUT" => LexemeCategory::LOGIC_OPERATOR,
        "ID" | "IDENTIFIER" => LexemeCategory::ID,
        "NUM" | "NUMBER" | "INTEGER" => LexemeCategory::INTEGER_CONST,
        _ => match name {
            "+" | "-" | "*" | "/" => LexemeCategory::NUMERIC_OPERATOR,
            "<" | "=" => LexemeCategory::COMPARE_OPERATOR,
            ":=" => LexemeCategory::ASSIGN_OPERATOR,
            "(" | ")" | ";" => LexemeCategory::LOGIC_OPERATOR,
            "identifier" | "id" => LexemeCategory::ID,
            "num" | "Number" => LexemeCategory::INTEGER_CONST,
            _ => LexemeCategory::LOGIC_OPERATOR,
        },
    }
}
