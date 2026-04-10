#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LexemeCategory {
    /// 整数常量（示例：`0`、`42`）
    INTEGER_CONST,
    /// 浮点常量（示例：`3.14`）
    FLOAT_CONST,
    /// 科学计数法常量（示例：`1.2e-3`）
    SCIENTIFIC_CONST,
    /// 数值运算符（`+` `-` `*` `/`）
    NUMERIC_OPERATOR,
    /// 注释 token
    NOTE,
    /// 字符串常量（示例：`"hello"`）
    STRING_CONST,
    /// 空白符（当前主要用于过滤，不向上层输出）
    SPACE_CONST,
    /// 比较运算符（示例：`<`、`=`）
    COMPARE_OPERATOR,
    /// 标识符（变量名等）
    ID,
    /// 逻辑/结构符号（当前包含部分界符及括号等）
    LOGIC_OPERATOR,
    /// 关键字（示例：`if`、`read`）
    KEYWORD,
    /// 赋值运算符（`:=`）
    ASSIGN_OPERATOR,
}

