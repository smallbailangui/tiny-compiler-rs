#![allow(non_camel_case_types)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LexemeCategory {
    INTEGER_CONST,
    FLOAT_CONST,
    SCIENTIFIC_CONST,
    NUMERIC_OPERATOR,
    NOTE,
    STRING_CONST,
    SPACE_CONST,
    COMPARE_OPERATOR,
    ID,
    LOGIC_OPERATOR,
    KEYWORD,
    ASSIGN_OPERATOR,
}
